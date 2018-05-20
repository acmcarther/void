#[macro_use]
extern crate log;

use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::SendError;
use std::ops::DerefMut;
use std::time::Duration;
use std::time::Instant;
use std::mem;

#[macro_export]
macro_rules! local_task_metadata {
  ($task_name:expr) => {
    TaskMetadata::raw_new($task_name.to_string(), file!().to_owned(), line!())
  }
}

pub struct SigKillDetails {
  event_description: String,
}

pub struct TaskArgs {
  pub sig_kill_receiver: Receiver<SigKillDetails>,
}

#[derive(Clone)]
pub enum TaskExitStatus {
  INTERRUPTED,
  FINISHED,
  UNKNOWN,
}

#[derive(Clone)]
pub struct TaskMetadata {
  pub task_name: String,
  pub file_path: Option<String>,
  pub line_number: Option<u32>,
}

pub struct RunOnceTask {
  task_metadata: TaskMetadata,
  boxed_task_fn: Box<FnMut(TaskArgs) -> TaskExitStatus + Send>,
}

pub struct RunningTask {
  task_metadata: TaskMetadata,
  task_join_handle: JoinHandle<()>,
  cached_result_opt: Option<TaskExitStatus>,
  sig_kill_sender: Sender<SigKillDetails>,
  result_receiver: Receiver<TaskExitStatus>,
}

pub struct TaskWaitParams {
  poll_interval: Duration,
}

impl Default for TaskMetadata {
  fn default() -> TaskMetadata {
    TaskMetadata {
      task_name: "unspecified".to_owned(),
      file_path: None,
      line_number: None,
    }
  }
}

impl TaskMetadata {
  /** Prefer `local_task_metadata` */
  pub fn raw_new(task_name: String, file_path: String, line_number: u32) -> TaskMetadata {
    TaskMetadata {
      task_name: task_name,
      file_path: Some(file_path),
      line_number: Some(line_number),
    }
  }
}

impl RunOnceTask {
  pub fn no_op_task() -> RunOnceTask {
    return RunOnceTask::new(local_task_metadata!("no_op"), |_unused_args| {
      TaskExitStatus::FINISHED
    });
  }

  pub fn new<F: FnOnce(TaskArgs) -> TaskExitStatus + Send + 'static>(
    task_metadata: TaskMetadata,
    boxed_task_fn: F,
  ) -> RunOnceTask {
    // This is a workaround for FnOnce not being boxable and FnBox not being stable
    let mut indirect_boxed_fn = Some(boxed_task_fn);
    let fn_mut_but_once = Box::new(move |task_args| (indirect_boxed_fn.take().unwrap())(task_args));

    RunOnceTask {
      task_metadata: task_metadata,
      boxed_task_fn: fn_mut_but_once,
    }
  }

  pub fn run(self) -> RunningTask {
    let (parent_sig_kill_sender, child_sig_kill_receiver) = mpsc::channel();
    let (child_result_sender, parent_result_receiver) = mpsc::channel();

    let task_args = TaskArgs {
      sig_kill_receiver: child_sig_kill_receiver,
    };

    let RunOnceTask {
      task_metadata,
      mut boxed_task_fn,
      ..
    } = self;
    let task_name = task_metadata.task_name.clone();
    let task_join_handle = thread::spawn(move || {
      let exit_status: TaskExitStatus = boxed_task_fn.deref_mut()(task_args);
      match child_result_sender.send(exit_status) {
        Ok(_) => {}
        Err(SendError(_)) => warn!("Parent hung up on task [{}]", task_name),
      }
    });

    RunningTask {
      task_metadata: task_metadata,
      cached_result_opt: None,
      task_join_handle: task_join_handle,
      sig_kill_sender: parent_sig_kill_sender,
      result_receiver: parent_result_receiver,
    }
  }
}

impl RunningTask {
  pub fn metadata(&self) -> &TaskMetadata {
    &self.task_metadata
  }

  pub fn is_done(&mut self) -> bool {
    match self.result_receiver.try_recv() {
      Ok(result) => {
        self.cached_result_opt = Some(result);
        true
      }
      Err(TryRecvError::Disconnected) => {
        warn!(
          "Task [{}]'s result sender hung up in `is_done`!",
          &self.task_metadata.task_name
        );
        true
      }
      Err(TryRecvError::Empty) => false,
    }
  }

  pub fn stop(self) -> thread::Result<TaskExitStatus> {
    // Swallowing Error -- It is OK if task has hung up.
    let _ = self.sig_kill_sender.send(SigKillDetails {
      event_description: "Called `stop` on RunningTask".to_owned(),
    });

    self.block()
  }

  pub fn block(mut self) -> thread::Result<TaskExitStatus> {
    let cached_result_opt = self.cached_result_opt.take();
    if let Some(cached_result) = cached_result_opt {
      return self.task_join_handle.join().map(|_unit| cached_result);
    } else {
      return self.pre_result_block();
    }
  }

  fn pre_result_block(self) -> thread::Result<TaskExitStatus> {
    let task_join_result = self.task_join_handle.join();

    match self.result_receiver.try_recv() {
      Ok(result) => task_join_result.map(|_unit| result),
      Err(TryRecvError::Disconnected) => {
        warn!(
          "Task [{}]'s result sender hung up in `pre_result_block`!",
          &self.task_metadata.task_name
        );
        task_join_result.map(|_unit| TaskExitStatus::UNKNOWN)
      }
      Err(TryRecvError::Empty) => {
        warn!(
          "Task [{}]'s result sender escaped in `pre_result_block!",
          &self.task_metadata.task_name
        );
        task_join_result.map(|_unit| TaskExitStatus::UNKNOWN)
      }
    }
  }
}

impl Default for TaskWaitParams {
  fn default() -> TaskWaitParams {
    TaskWaitParams {
      poll_interval: Duration::from_millis(1000),
    }
  }
}

pub fn wait_for_tasks(
  task_wait_params: TaskWaitParams,
  tasks: Vec<RunningTask>,
) -> Vec<thread::Result<TaskExitStatus>> {
  let mut running_tasks = tasks
    .into_iter()
    .enumerate()
    .map(|v| Some(v))
    .collect::<Vec<_>>();
  let mut finished_tasks = Vec::new();

  while finished_tasks.len() != running_tasks.len() {
    let check_start = Instant::now();
    for mut task_with_idx_opt in running_tasks.iter_mut() {
      if task_with_idx_opt.is_none() {
        continue;
      }

      let is_done = task_with_idx_opt
        .as_mut()
        .map(|&mut (_, ref mut task)| task.is_done())
        .unwrap_or(false);

      if is_done {
        let mut swap_task_with_idx_opt = None;
        mem::swap(&mut swap_task_with_idx_opt, task_with_idx_opt);

        // UNWRAP: Known to exist from `is_done`.
        finished_tasks.push(swap_task_with_idx_opt.unwrap());
      }
    }

    let check_duration = Instant::now().duration_since(check_start);
    let sleep_time_remaining_opt = task_wait_params.poll_interval.checked_sub(check_duration);

    if let Some(sleep_time_remaining) = sleep_time_remaining_opt {
      thread::sleep(sleep_time_remaining)
    }
  }

  finished_tasks.sort_by_key(|&(idx, _)| idx.clone());

  finished_tasks
    .into_iter()
    .map(|(_, task)| task.block())
    .collect()
}

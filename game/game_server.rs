extern crate log;
extern crate task;

use task::RunOnceTask;

pub fn new_task() -> RunOnceTask {
  RunOnceTask::no_op_task()
}

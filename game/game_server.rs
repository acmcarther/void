#[macro_use]
extern crate log;
#[macro_use]
extern crate task;

use std::sync::mpsc::Receiver;
use task::RunOnceTask;
use task::SigKillDetails;
use task::TaskExitStatus;

pub struct GameServerParams {}

pub struct LateBoundGameServerParams {
  pub sig_kill_receiver: Receiver<SigKillDetails>,
}

pub struct EarlyGameServer {
  params: GameServerParams,
}

pub struct GameServer {
  params: GameServerParams,
  late_bound_params: LateBoundGameServerParams,
}

impl Default for GameServerParams {
  fn default() -> GameServerParams {
    GameServerParams {}
  }
}

impl EarlyGameServer {
  pub fn new(params: GameServerParams) -> EarlyGameServer {
    EarlyGameServer { params: params }
  }

  pub fn into_game_server(self, late_bound_params: LateBoundGameServerParams) -> GameServer {
    GameServer {
      params: self.params,
      late_bound_params: late_bound_params,
    }
  }
}

impl GameServer {
  pub fn run_main_loop(&mut self) -> TaskExitStatus {
    info!("Starting server main loop");

    TaskExitStatus::UNKNOWN
  }
}

pub fn new_task() -> RunOnceTask {
  let early_game_server = EarlyGameServer::new(GameServerParams::default());

  RunOnceTask::new(local_task_metadata!("game_server"), |task_args| {
    let mut game_server = early_game_server.into_game_server(LateBoundGameServerParams {
      sig_kill_receiver: task_args.sig_kill_receiver,
    });

    game_server.run_main_loop()
  })
}

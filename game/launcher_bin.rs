#![feature(used)]
extern crate game_client;
extern crate game_server;
extern crate init;
#[macro_use]
extern crate lazy_static;
extern crate task;
#[macro_use]
extern crate zcfg;

use task::TaskWaitParams;

define_cfg!(
  enabled_launcher_components,
  Vec<String>,
  vec!["client".to_owned(), "server".to_owned()],
  "Which launchable components to run."
);

fn main() {
  init::init();

  let components = enabled_launcher_components::CONFIG
    .get_value()
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect::<Vec<_>>();
  verify_components(&components);

  let all_runnable_tasks = {
    let mut v = Vec::new();
    if components.contains(&"client".to_owned()) {
      v.push(game_client::new_task());
    }

    if components.contains(&"server".to_owned()) {
      v.push(game_server::new_task());
    }
    v
  };

  let all_running_tasks = all_runnable_tasks
    .into_iter()
    .map(|t| t.run())
    .collect::<Vec<_>>();

  task::wait_for_tasks(TaskWaitParams::default(), all_running_tasks);
}

fn verify_components(components: &Vec<String>) {
  for component in components {
    match component.as_str() {
      "client" | "server" => {}
      something_else => {
        panic!(
          "\"{}\" is not a known launchable component.",
          something_else
        );
      }
    }
  }
}

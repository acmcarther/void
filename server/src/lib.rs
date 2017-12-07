/** A game backend installation in a server cluster. */
struct ServerNode;

impl ServerNode {
  pub fn initialized() -> ServerNode {
    ServerNode
  }

  pub fn run_forever(self) {
    let keep_running = true;

    while keep_running {
      std::thread::sleep(std::time::Duration::from_secs(2));
    }
  }
}


pub fn run() {
  let node = ServerNode::initialized();

  node.run_forever()
}

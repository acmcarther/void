/**
 * Id for a particular client.
 *
 * Durable across disconnect/reconnect, provided that the client reconnects with the same
 * parameters.
 */
pub type ClientId = u64;

/** Description of a Server to connect to. */
pub struct ServerSpec {
  pub address: String,
}

/**
 * Describes how a packet should be handled in the event of a drop
 *
 * `OneShot` indicates that the packet will only be transmitted once.
 * `Retry` indicates that the packet should be retried until client disconnect or ACK.
 */
pub enum TransmissionPolicy {
  OneShot,
  Retry,
}

pub trait NetcodeClient {
  /**
   * Retrieves this client's understanding of the current status of the connection.
   *
   * This state is time-sensitive, and may no longer be correct by the time it is returned.
   * If the connection is in an ended state, a new client must be created. New clients should be
   * created with the same parameters as old clients so that the server recognizes them again.
   */
  fn get_connection_status(&self) -> ConnectionStatus;

  /**
   * Enqueues a packet to be emitted on the next call of process_packets.
   * 
   * If the connection is in an ended state, this function will have no effect.
   */
  fn schedule_send_packet(&self, payload_bytes: Vec<u8>, transmission_policy: TransmissionPolicy);

  /**
   * Emits all enqueued packets, and retrieves packets to be processed.
   *
   * Returns whether or not there are packets to be processed.
   */
  fn process_packets(&mut self) -> bool;

  /** Fetched packets to be processed. */
  fn retrieve_packets(&mut self) -> Vec<Vec<u8>>;

  /** Returns the underlying client object. */
  unsafe fn raw_client(&self) -> &netcode_client_t;

  /** Returns the underlying client object mutably. */
  unsafe fn raw_client_mut(&mut self) -> &mut netcode_client_t;
}

pub trait NetcodeServer {
  /** Yields all known clients */
  fn enumerate_clients(&self) -> std::iter::Iter<&ClientId>;

  /** Yields all active clients */
  fn enumerate_active_clients(&self) -> std::iter::Iter<&ClientId>;

  /**
   * Retrieves this servers's understanding of the current status of a client the connection.
   *
   * This state is time-sensitive, and may no longer be correct by the time it is returned.
   * If the connection is in an ended state, a new client must be created. New clients should be
   * created with the same parameters as old clients so that the server recognizes them again.
   */
  fn get_connection_status(&self, client_id: &ClientId) -> Option<&ConnectionStatus>;

  /**
   * Enqueues a packet to be emitted to a particular client.
   *
   * If a client is disconnected, this method will do nothing.
   */
  fn schedule_packet(&mut self, client_id: &ClientId, payload_bytes: Vec<u8>, transmission_policy: TransmissionPolicy);

  /**
   * Emits all pending packets, and retrieves new packets ready to be fetched with
   * retrieve_packets.
   */
  fn process_packets(&mut self);

  /** Fetches packets to be processed for a client. */
  fn retrieve_packets(&mut self, client_id: &ClientId) -> Vec<Vec<u8>>;

  /** Returns an underlying connection object. */
  unsafe fn raw_connection(&self, client_id: &ClientId) -> Option<&reliable_endpoint_t>;

  /** Returns an underlying connection object mutably. */
  unsafe fn raw_connection_mut(&mut self) -> Option<&mut reliable_endpoint_t>;

  /** Returns the underlying server object. */
  unsafe fn raw_server(&self) -> &netcode_server_t;

  /** Returns the underlying server object mutably. */
  unsafe fn raw_server_mut(&mut self) -> &mut netcode_server_t;
}

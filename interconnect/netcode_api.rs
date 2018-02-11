extern crate netcode_io_sys as nio;
extern crate reliable_io_sys as rio;

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

pub struct ConnectionStatus;

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
  unsafe fn raw_client(&self) -> &nio::netcode_client_t;

  /** Returns the underlying client object mutably. */
  unsafe fn raw_client_mut(&mut self) -> &mut nio::netcode_client_t;
}

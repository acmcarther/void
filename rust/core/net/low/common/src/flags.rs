define_pub_cfg!(
  nio_server_addr,
  String,
  "127.0.0.1:40000".to_owned(),
  "ADDRESS:PORT for the netcode io server"
);

define_pub_cfg!(
  max_connection_count,
  u32,
  16u32,
  "Max number of accepted connections"
);

define_pub_cfg!(
  max_unfragmented_packet_size,
  u32,
  ::nio::NETCODE_MAX_PACKET_SIZE as u32,
  "Maximum size of an unfragmented packet (else fragmentation required)."
);

use std::net::TcpListener;

pub(crate) fn get_free_port() -> Option<u16> {
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  return match listener.local_addr() {
    Ok(addr) => Some(addr.port()),
    Err(_) => None,
  };
}

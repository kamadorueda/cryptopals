use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn bytes_to_base64(bytes: &[u8]) -> String {
  STANDARD.encode(bytes)
}

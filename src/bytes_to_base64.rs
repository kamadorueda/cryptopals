use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;

pub fn bytes_to_base64(bytes: &[u8]) -> String {
  STANDARD_NO_PAD.encode(bytes)
}

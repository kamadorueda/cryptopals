use anyhow::Context;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn base64_to_bytes(
  base64: &str,
) -> anyhow::Result<Vec<u8>> {
  STANDARD
    .decode(base64)
    .map_err(|err| anyhow::anyhow!(err))
    .with_context(|| format!("bytes_to_base64({base64:?})"))
}

use anyhow::anyhow;
use anyhow::Context;

pub fn hex_to_bytes(hex: &str) -> anyhow::Result<Vec<u8>> {
  hex::decode(hex)
    .map_err(|err| anyhow!(err))
    .with_context(|| format!("hex_to_bytes({hex:?})"))
}

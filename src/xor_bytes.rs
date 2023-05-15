use std::iter::zip;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> anyhow::Result<Vec<u8>> {
  anyhow::ensure!(a.len() == b.len(), "Both slices must be the same length");

  Ok(zip(a.iter(), b.iter()).map(|(a, b)| a ^ b).collect())
}

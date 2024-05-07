use std::iter::zip;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> anyhow::Result<Vec<u8>> {
  anyhow::ensure!(a.len() == b.len(), "Both slices must be the same length");

  Ok(zip(a.iter(), b.iter()).map(|(a, b)| a ^ b).collect())
}

pub fn xor_bytes_const<const N: usize>(mut a: [u8; N], b: [u8; N]) -> [u8; N] {
  a.iter_mut().zip(b).for_each(|(a, b)| *a ^= b);

  a
}

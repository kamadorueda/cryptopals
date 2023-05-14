use std::iter::zip;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
  debug_assert!(a.len() == b.len());

  zip(a.iter(), b.iter()).map(|(a, b)| a ^ b).collect()
}

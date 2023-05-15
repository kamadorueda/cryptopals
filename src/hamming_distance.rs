use std::iter::zip;

pub fn hamming_distance(
  a: &[u8],
  b: &[u8],
) -> anyhow::Result<u32> {
  anyhow::ensure!(
    a.len() == b.len(),
    "Both strings must be the same length"
  );

  Ok(zip(a, b).map(|(a, b)| (a ^ b).count_ones()).sum())
}

#[cfg(test)]
#[test]
fn test() -> anyhow::Result<()> {
  assert_eq!(
    hamming_distance(b"this is a test", b"wokka wokka!!!")?,
    37
  );

  Ok(())
}

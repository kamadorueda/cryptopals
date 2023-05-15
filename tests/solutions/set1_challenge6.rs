use cryptopals::hamming_distance::hamming_distance;

/// Find the key length by finding the normalized hamming
/// distance between each chunk of bytes of the given
/// length.
fn find_key_len(input: &[u8]) -> anyhow::Result<usize> {
  let mut key_lens: Vec<(usize, f64)> = (2..40)
    .map(|key_len| {
      let chunks = input.len() / key_len - 2;

      let hamming_distance = (0..chunks).fold(
        Ok(0),
        |sum, chunk| {
          let sum = sum?;

          let hamming_distance = hamming_distance(
            &input[chunk * key_len
              ..(1 + chunk) * key_len],
            &input[(1 + chunk) * key_len
              ..(2 + chunk) * key_len],
          )?;

          Ok(sum + hamming_distance)
        },
      )?;

      let normalized_hamming_distance = hamming_distance
        as f64
        / key_len as f64
        / chunks as f64;

      Ok((key_len, normalized_hamming_distance))
    })
    .collect::<anyhow::Result<_>>()?;

  // Sort by normalized hamming distance
  key_lens.sort_by(|(_, a), (_, b)| a.total_cmp(b));

  Ok(key_lens.swap_remove(0).0)
}

pub fn solve(
  key_len: usize,
  input: &[u8],
) -> anyhow::Result<Vec<u8>> {
  // Transpose the blocks so that the first block contains
  // the first byte of every block, the second block
  // contains the second byte of every block, etc.
  let blocks: Vec<Vec<u8>> = (0..key_len)
    .map(|block| {
      input
        .iter()
        .enumerate()
        .filter(|(i, _)| i % key_len == block)
        .map(|(_, byte)| *byte)
        .collect::<Vec<u8>>()
    })
    .collect();

  // Solve each block as if it were single-character XOR
  let keys: Vec<u8> = blocks
    .into_iter()
    .map(|block| {
      Ok(
        crate::solutions::set1_challenge3::solve(&block)?
          .into_iter()
          .map(|solution| solution.key)
          .next(),
      )
    })
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .collect();

  Ok(keys)
}

#[cfg(test)]
#[test]
fn solution() -> anyhow::Result<()> {
  use cryptopals::base64_to_bytes::base64_to_bytes;

  let input = include_str!("set1_challenge6_input.txt")
    .lines()
    .collect::<String>();
  let input = base64_to_bytes(&input)?;

  // Find the key length
  let key_len = find_key_len(&input)?;

  assert_eq!(key_len, 29);

  // Find the key
  let key = solve(key_len, &input)?;

  assert_eq!(
    &String::from_utf8_lossy(&key),
    // Small mistake on the `b` in `the`
    "Terminator X: Bring thb noise"
  );

  // Fix the key manually and decrypt
  let plaintext = crate::solutions::set1_challenge5::solve(
    b"Terminator X: Bring the noise",
    &input,
  )?;

  assert_eq!(
    &String::from_utf8_lossy(&plaintext)[0..33],
    "I'm back and I'm ringin' the bell"
  );

  Ok(())
}

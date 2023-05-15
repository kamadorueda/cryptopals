use cryptopals::hamming_distance::hamming_distance;

fn find_key_len(input: &[u8]) -> anyhow::Result<usize> {
  let mut key_lens: Vec<(usize, f64)> = (2..40)
    .into_iter()
    .map(|key_len| {
      let chunks = input.len() / key_len - 2;

      let hamming_distance = (0..chunks).into_iter().fold(
        Ok(0),
        |sum, chunk| {
          let sum = sum?;

          let hamming_distance = hamming_distance(
            &input[(0 + chunk) * key_len
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
  key_lens.sort_by(|(_, a), (_, b)| a.total_cmp(&b));

  Ok(key_lens.swap_remove(0).0)
}

// pub fn solve(input: &str) -> anyhow::Result<String> {
//   let input_bytes = input.as_bytes();
//   let key_bytes = b"ICE"
//     .iter()
//     .cycle()
//     .copied()
//     .take(input_bytes.len())
//     .collect::<Vec<u8>>();

//   let encrypted_bytes = xor_bytes(input_bytes,
// &key_bytes)?;

//   Ok(bytes_to_hex(&encrypted_bytes))
// }

#[cfg(test)]
#[test]
fn solution() -> anyhow::Result<()> {
  use cryptopals::base64_to_bytes::base64_to_bytes;

  let input = include_str!("set1_challenge6_input.txt")
    .lines()
    .collect::<String>();
  let input = base64_to_bytes(&input)?;

  let key_len = find_key_len(&input)?;

  assert_eq!(key_len, 29);

  todo!();

  Ok(())
}

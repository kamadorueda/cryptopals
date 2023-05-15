use std::collections::HashSet;

use cryptopals::bytes_to_hex::bytes_to_hex;
use cryptopals::hex_to_bytes::hex_to_bytes;

fn is_ecb(block_size_in_bytes: usize, input: &[u8]) -> bool {
  let chunks: HashSet<&[u8]> =
    input.chunks_exact(block_size_in_bytes).collect();

  input.len() / block_size_in_bytes > chunks.len()
}

#[test]
fn solution() -> anyhow::Result<()> {
  let input = include_str!("set1_challenge8_input.txt").lines();
  let block_size_in_bytes = 16;

  assert_eq!(
    input
      .into_iter()
      .flat_map(hex_to_bytes)
      .filter(|bytes| { is_ecb(block_size_in_bytes, bytes) })
      .map(|bytes| bytes_to_hex(&bytes))
      .next()
      .as_deref(),
    Some(
      "d880619740a8a19b7840a8a31c810a3d\
       08649af70dc06f4fd5d2d69c744cd283\
       e2dd052f6b641dbf9d11b0348542bb57\
       08649af70dc06f4fd5d2d69c744cd283\
       9475c9dfdbc1d46597949d9c7e82bf5a\
       08649af70dc06f4fd5d2d69c744cd283\
       97a93eab8d6aecd566489154789a6b03\
       08649af70dc06f4fd5d2d69c744cd283\
       d403180c98c8f6db1f2a3f9c4040deb0\
       ab51b29933f2c123c58386b06fba186a"
    )
  );

  Ok(())
}

use cryptopals::bytes_to_hex::bytes_to_hex;
use cryptopals::xor_bytes::xor_bytes;

pub fn solve(
  key: &[u8],
  input: &[u8],
) -> anyhow::Result<Vec<u8>> {
  let key_bytes = key
    .iter()
    .cycle()
    .copied()
    .take(input.len())
    .collect::<Vec<u8>>();

  let encrypted_bytes = xor_bytes(input, &key_bytes)?;

  Ok(encrypted_bytes)
}

#[cfg(test)]
#[test]
fn solution() -> anyhow::Result<()> {
  let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

  assert_eq!(
    bytes_to_hex(&solve(b"ICE", input)?),
    "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
  );

  Ok(())
}

use cryptopals::bytes_to_hex::bytes_to_hex;
use cryptopals::xor_bytes::xor_bytes;

pub fn solve(input: &str) -> anyhow::Result<String> {
  let input_bytes = input.as_bytes();
  let key_bytes = b"ICE"
    .iter()
    .cycle()
    .copied()
    .take(input_bytes.len())
    .collect::<Vec<u8>>();

  let encrypted_bytes = xor_bytes(input_bytes, &key_bytes)?;

  Ok(bytes_to_hex(&encrypted_bytes))
}

#[cfg(test)]
#[test]
fn solution() -> anyhow::Result<()> {
  let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

  assert_eq!(
    solve(&input)?,
    "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
  );

  Ok(())
}

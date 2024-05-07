use std::iter::repeat;

#[test]
fn solution() -> anyhow::Result<()> {
  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE", 20)?,
    b"YELLOW SUBMARINE\x04\x04\x04\x04",
  );

  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE  ", 20)?,
    b"YELLOW SUBMARINE  \x02\x02",
  );

  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE    ", 20)?,
    b"YELLOW SUBMARINE    ",
  );

  Ok(())
}

fn pkcs7_pad(input: &[u8], block_size: usize) -> anyhow::Result<Vec<u8>> {
  let input_len = input.len();

  anyhow::ensure!(input_len <= block_size);

  let padding = block_size - input_len;

  anyhow::ensure!(padding < u8::MAX as usize);

  Ok(
    input
      .iter()
      .copied()
      .chain(repeat(padding as u8))
      .take(block_size)
      .collect(),
  )
}

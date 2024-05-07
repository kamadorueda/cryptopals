use std::iter::repeat;

use anyhow::Context;

pub fn pkcs7_pad(
  mut input: Vec<u8>,
  block_size: u8,
) -> anyhow::Result<Vec<u8>> {
  let block_size = block_size as usize;

  let last_block_len = input.len() % block_size;

  let padding = if last_block_len == 0 {
    block_size
  } else {
    block_size - last_block_len
  };

  anyhow::ensure!(padding < u8::MAX as usize);

  input.extend(repeat(padding as u8).take(padding));

  Ok(input)
}

pub fn pkcs7_unpad(mut input: Vec<u8>) -> anyhow::Result<Vec<u8>> {
  let input_len = input.len();

  let padding = *input.last().context("Invalid PKCS#7 padding")?;

  anyhow::ensure!(
    input
      .split_off(input_len - padding as usize)
      .into_iter()
      .all(|byte| byte == padding)
  );

  Ok(input)
}

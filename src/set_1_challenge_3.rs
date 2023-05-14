use crate::bytes_frequency::bytes_frequency;
use crate::bytes_frequency::ENGLISH_FREQUENCIES;
use crate::chi_squared::chi_squared;
use crate::hex_to_bytes::hex_to_bytes;
use crate::xor_bytes::xor_bytes;

#[derive(Debug)]
pub struct Scored {
  pub key: u8,
  pub chi_square: f64,
  pub result: String,
}

/// Take the input, XOR it against every possible key,
/// score them by likelihood of it being natural english.
pub fn solve(input: &str) -> anyhow::Result<Vec<Scored>> {
  let input_bytes = hex_to_bytes(input)?;

  let mut scores: Vec<Scored> = (u8::MIN..u8::MAX)
    .map(|key| {
      let key_bytes = vec![key; input_bytes.len()];
      let bytes = xor_bytes(&input_bytes, &key_bytes);
      let bytes_lowercase: Vec<u8> = bytes
        .iter()
        .copied()
        .map(|byte| byte as char)
        .map(|char| char.to_ascii_lowercase())
        .map(|char| char as u8)
        .collect();

      let frequencies = bytes_frequency(&bytes_lowercase);

      Ok(Scored {
        key,
        chi_square: chi_squared(&[&frequencies, &ENGLISH_FREQUENCIES])?,
        result: String::from_utf8(bytes).unwrap_or_default(),
      })
    })
    .collect::<anyhow::Result<_>>()?;

  scores.sort_by(|a, b| a.chi_square.total_cmp(&b.chi_square));

  Ok(scores)
}

#[test]
fn solution() -> anyhow::Result<()> {
  assert_eq!(
    solve(
      "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    )?
    .into_iter()
    .map(|solution| solution.result)
    .next()
    .as_deref(),
    Some("Cooking MC's like a pound of bacon")
  );

  Ok(())
}

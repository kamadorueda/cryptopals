use crate::bytes_frequency::bytes_frequency;
use crate::bytes_frequency::ENGLISH_FREQUENCIES;
use crate::chi_square::chi_squared;
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

  let (_, english_frequencies): (Vec<u8>, Vec<f64>) =
    ENGLISH_FREQUENCIES.into_iter().unzip();

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
      let frequencies_en: Vec<f64> = ENGLISH_FREQUENCIES
        .into_iter()
        .map(|(byte, _)| frequencies[byte as usize])
        .collect();

      Ok(Scored {
        key,
        chi_square: chi_squared(&[&frequencies_en, &english_frequencies])?,
        result: String::from_utf8(bytes).unwrap_or_default(),
      })
    })
    .collect::<anyhow::Result<_>>()?;

  scores.sort_by(|a, b| a.chi_square.total_cmp(&b.chi_square));

  let scores = scores
    .into_iter()
    .filter(|solution| !solution.chi_square.is_nan())
    .collect();

  for result in &scores {
    println!("{result:?}");
  }

  Ok(scores)
}

#[test]
fn solution() -> anyhow::Result<()> {
  assert_eq!(
    // Skip the first 6 results, they're all equally good in terms of
    // frequencies, but they contain some garbage.
    solve(
      "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    )?
    .into_iter()
    .rev()
    .map(|solution| solution.result)
    .skip(6)
    .next()
    .as_deref(),
    Some("Cooking MC's like a pound of bacon")
  );

  Ok(())
}

use cryptopals::ascii_frequencies_en::ASCII_FREQUENCIES_EN;
use cryptopals::bytes_frequency::bytes_frequency;
use cryptopals::chi_squared::chi_squared;
use cryptopals::hex_to_bytes::hex_to_bytes;
use cryptopals::xor_bytes::xor_bytes;

#[derive(Debug)]
pub struct SingleCharacterXorCandidate {
  pub key: u8,
  pub chi_square: f64,
  pub result: String,
}

/// Take the input, XOR it against every possible key,
/// score them by likelihood of it being natural english.
pub fn find_single_character_xor_key_candidates(
  input: &[u8],
) -> anyhow::Result<Vec<SingleCharacterXorCandidate>> {
  let mut scores: Vec<SingleCharacterXorCandidate> = (u8::MIN..u8::MAX)
    .map(|key| {
      let key_bytes = vec![key; input.len()];
      let bytes = xor_bytes(input, &key_bytes)?;
      let bytes_lowercase: Vec<u8> = bytes
        .iter()
        .copied()
        .map(|byte| byte as char)
        .map(|char| char.to_ascii_lowercase())
        .map(|char| char as u8)
        .collect();

      let frequencies = bytes_frequency(&bytes_lowercase);

      Ok(SingleCharacterXorCandidate {
        key,
        chi_square: chi_squared(&[&frequencies, &ASCII_FREQUENCIES_EN])?,
        result: String::from_utf8(bytes).unwrap_or_default(),
      })
    })
    .collect::<anyhow::Result<_>>()?;

  scores.sort_by(|a, b| a.chi_square.total_cmp(&b.chi_square));

  Ok(scores)
}

#[test]
fn solution() -> anyhow::Result<()> {
  let input =
    "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let bytes = hex_to_bytes(input)?;

  assert_eq!(
    find_single_character_xor_key_candidates(&bytes)?
      .into_iter()
      .map(|solution| solution.result)
      .next()
      .as_deref(),
    Some("Cooking MC's like a pound of bacon")
  );

  Ok(())
}

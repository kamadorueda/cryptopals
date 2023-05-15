use cryptopals::hex_to_bytes::hex_to_bytes;

use crate::solutions::set1_challenge3::find_single_character_xor_key_candidates;

#[test]
fn solution() -> anyhow::Result<()> {
  let inputs: Vec<Vec<u8>> = include_str!("set1_challenge4_input.txt")
    .lines()
    .map(hex_to_bytes)
    .collect::<anyhow::Result<_>>()?;

  // Score all inputs by their likelihood of it being
  // natural english using the solver from the previous
  // challenge.
  let mut candidates: Vec<_> = inputs
    .iter()
    .map(|input| find_single_character_xor_key_candidates(input))
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .collect();

  candidates.sort_by(|a, b| a.chi_square.total_cmp(&b.chi_square));

  // 327 inputs, 256 possible keys each
  assert_eq!(candidates.len(), 83385);

  assert_eq!(
    candidates
      .into_iter()
      .map(|candidate| candidate.result)
      .next()
      .as_deref(),
    Some("Now that the party is jumping\n")
  );

  Ok(())
}

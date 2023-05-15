use cryptopals::hex_to_bytes::hex_to_bytes;

/// Score all inputs by their likelihood of it being natural
/// english using the solver from the previous challenge.
pub fn solve(
  inputs: &[Vec<u8>],
) -> anyhow::Result<
  Vec<crate::solutions::set1_challenge3::Scored>,
> {
  let mut scores: Vec<_> = inputs
    .iter()
    .map(|input| {
      crate::solutions::set1_challenge3::solve(input)
    })
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .collect();

  scores
    .sort_by(|a, b| a.chi_square.total_cmp(&b.chi_square));

  Ok(scores)
}

#[cfg(test)]
#[test]
fn solution() -> anyhow::Result<()> {
  let inputs: Vec<Vec<u8>> =
    include_str!("set1_challenge4_input.txt")
      .lines()
      .map(hex_to_bytes)
      .collect::<anyhow::Result<_>>()?;

  let candidates = solve(&inputs)?;

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

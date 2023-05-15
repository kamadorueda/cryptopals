/// Score all inputs by their likelihood of it being natural
/// english using the solver from the previous challenge.
pub fn solve(
  inputs: &[&str],
) -> anyhow::Result<
  Vec<crate::solutions::set1_challenge3::Scored>,
> {
  let mut scores: Vec<_> = inputs
    .into_iter()
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
  let inputs: Vec<&str> =
    include_str!("set1_challenge4_input.txt")
      .lines()
      .collect();

  let candidates = solve(&inputs)?;

  // 327 inputs, 256 possible keys each
  assert_eq!(candidates.len(), 83385);

  assert_eq!(
    // Pick the 3rd best candidate
    candidates
      .into_iter()
      .map(|candidate| candidate.result)
      .nth(3)
      .as_deref(),
    Some("Now that the party is jumping\n")
  );

  Ok(())
}

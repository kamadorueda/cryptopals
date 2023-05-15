use anyhow::Context;
use statrs::distribution::ChiSquared;
use statrs::distribution::ContinuousCDF;

pub fn chi_squared(inputs: &[&[f64]]) -> anyhow::Result<f64> {
  let rows = inputs.len();
  anyhow::ensure!(rows > 0, "There must be at least one row");

  let columns = inputs[0].len();
  anyhow::ensure!(columns > 0, "There must be at least one column");

  anyhow::ensure!(
    inputs.iter().all(|row| row.len() == columns),
    "All rows must have the same number of columns"
  );

  let column_totals: Vec<f64> = (0..columns)
    .map(|column| inputs.iter().fold(0f64, |sum, row| sum + row[column]))
    .collect();
  let row_totals: Vec<f64> =
    (0..rows).map(|row| inputs[row].iter().sum()).collect();

  let total = column_totals.iter().sum::<f64>();

  let test_statistic = row_totals
    .iter()
    .copied()
    .enumerate()
    .map(|(row, row_total)| {
      column_totals
        .iter()
        .copied()
        .enumerate()
        .map(|(column, column_total)| {
          let observed = inputs[row][column];

          if total == 0.0 || column_total == 0.0 || row_total == 0.0 {
            if observed == 0.0 { 1.0 } else { 0.0 }
          } else {
            let expected = column_total * row_total / total;

            (observed - expected).powf(2f64) / expected
          }
        })
        .sum::<f64>()
    })
    .sum::<f64>();

  let degrees_of_freedom = (rows - 1) * (columns - 1);

  let chi_squared = ChiSquared::new(degrees_of_freedom as f64)
    .map_err(|err| anyhow::anyhow!(err))
    .context("Unable to instantiate ChiSquared distribution")?;

  let p = 1f64 - chi_squared.cdf(test_statistic);

  Ok(p)
}

#[cfg(test)]
#[test]
fn test() -> anyhow::Result<()> {
  use statrs::assert_almost_eq;

  assert_almost_eq!(
    chi_squared(&[&[120.0, 90.0, 40.0], &[110.0, 95.0, 45.0]])?,
    0.649198,
    0.000001
  );

  assert_almost_eq!(
    chi_squared(&[&[0.0, 0.0], &[0.0, 0.0]])?,
    0.045500,
    0.000001
  );

  Ok(())
}

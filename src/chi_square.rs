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
    .into_iter()
    .map(|column| inputs.iter().fold(0f64, |sum, row| sum + row[column]))
    .collect();
  let row_totals: Vec<f64> = (0..rows)
    .into_iter()
    .map(|row| inputs[row].iter().sum())
    .collect();

  let total = column_totals.iter().sum::<f64>();

  let test_statistic = row_totals
    .iter()
    .enumerate()
    .map(|(row, row_total)| {
      column_totals
        .iter()
        .enumerate()
        .map(|(column, column_total)| {
          let expected = column_total * row_total / total;
          let observed = inputs[row][column];

          (observed - expected).powf(2f64) / expected
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

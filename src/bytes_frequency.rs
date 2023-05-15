pub fn bytes_frequency(bytes: &[u8]) -> [f64; 256] {
  let len = bytes.len() as f64;

  let mut frequencies = [0f64; 256];

  bytes
    .iter()
    .for_each(|byte| frequencies[*byte as usize] += 1f64);

  frequencies
    .iter_mut()
    .for_each(|frequency| *frequency /= len);

  frequencies
}

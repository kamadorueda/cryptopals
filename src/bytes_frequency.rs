pub const ENGLISH_FREQUENCIES: [(u8, f64); 26] = [
  (b'a', 0.082),
  (b'b', 0.015),
  (b'c', 0.028),
  (b'd', 0.043),
  (b'e', 0.13),
  (b'f', 0.022),
  (b'g', 0.02),
  (b'h', 0.061),
  (b'i', 0.07),
  (b'j', 0.0015),
  (b'k', 0.0077),
  (b'l', 0.04),
  (b'm', 0.024),
  (b'n', 0.067),
  (b'o', 0.075),
  (b'p', 0.019),
  (b'q', 0.00095),
  (b'r', 0.06),
  (b's', 0.063),
  (b't', 0.091),
  (b'u', 0.028),
  (b'v', 0.0098),
  (b'w', 0.024),
  (b'x', 0.0015),
  (b'y', 0.02),
  (b'z', 0.00074),
];

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

use cryptopals::aes_128::aes_128_decrypt;
use cryptopals::base64_to_bytes::base64_to_bytes;
use tap::Pipe as _;

#[test]
fn solution() -> anyhow::Result<()> {
  let input = include_str!("set1_challenge7_input.txt")
    .lines()
    .collect::<String>()
    .pipe_as_ref(base64_to_bytes)?;

  let output = aes_128_decrypt(*b"YELLOW SUBMARINE", &input)?
    .pipe(String::from_utf8)
    .map_err(|err| anyhow::anyhow!(err))?;

  assert_eq!(&output, include_str!("set1_challenge7_output.txt"));

  Ok(())
}

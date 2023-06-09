use cryptopals::bytes_to_base64::bytes_to_base64;
use cryptopals::hex_to_bytes::hex_to_bytes;

#[test]
fn solution() -> anyhow::Result<()> {
  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let bytes = hex_to_bytes(input)?;
  let base64 = bytes_to_base64(&bytes);

  assert_eq!(
    &base64,
    "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
  );

  Ok(())
}

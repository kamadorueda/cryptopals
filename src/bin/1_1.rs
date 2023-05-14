use cryptopals::bytes_to_base64::bytes_to_base64;
use cryptopals::hex_to_bytes::hex_to_bytes;

fn main() -> anyhow::Result<()> {
  let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let bytes = hex_to_bytes(hex)?;
  let base64 = bytes_to_base64(&bytes);

  println!("{base64}");

  Ok(())
}

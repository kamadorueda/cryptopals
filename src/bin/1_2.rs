use cryptopals::bytes_to_hex::bytes_to_hex;
use cryptopals::hex_to_bytes::hex_to_bytes;
use cryptopals::xor_bytes::xor_bytes;

fn main() -> anyhow::Result<()> {
  let a = "1c0111001f010100061a024b53535009181c";
  let b = "686974207468652062756c6c277320657965";
  let a_bytes = hex_to_bytes(a)?;
  let b_bytes = hex_to_bytes(b)?;
  let bytes = xor_bytes(&a_bytes, &b_bytes);
  let hex = bytes_to_hex(&bytes);

  println!("{hex}");

  Ok(())
}

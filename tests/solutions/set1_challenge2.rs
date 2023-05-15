use cryptopals::bytes_to_hex::bytes_to_hex;
use cryptopals::hex_to_bytes::hex_to_bytes;
use cryptopals::xor_bytes::xor_bytes;

#[test]
fn solution() -> anyhow::Result<()> {
  let inputs = &[
    "1c0111001f010100061a024b53535009181c",
    "686974207468652062756c6c277320657965",
  ];
  let a_bytes = hex_to_bytes(&inputs[0])?;
  let b_bytes = hex_to_bytes(&inputs[1])?;
  let bytes = xor_bytes(&a_bytes, &b_bytes)?;
  let hex = bytes_to_hex(&bytes);

  assert_eq!(&hex, "746865206b696420646f6e277420706c6179");

  Ok(())
}

use cryptopals::pkcs7::pkcs7_pad;

#[test]
fn solution() -> anyhow::Result<()> {
  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE".to_vec(), 20)?,
    b"YELLOW SUBMARINE\x04\x04\x04\x04",
  );

  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE".to_vec(), 10)?,
    b"YELLOW SUBMARINE\x04\x04\x04\x04",
  );

  assert_eq!(
    &pkcs7_pad(b"YELLOW SUBMARINE  ".to_vec(), 20)?,
    b"YELLOW SUBMARINE  \x02\x02",
  );

  assert_eq!(
    &pkcs7_pad(b"YELLOW".to_vec(), 6)?,
    b"YELLOW\x06\x06\x06\x06\x06\x06",
  );

  Ok(())
}

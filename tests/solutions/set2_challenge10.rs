use aes::Aes128;
use cipher::generic_array::GenericArray;
use cipher::BlockDecrypt as _;
use cipher::KeyInit as _;
use cryptopals::base64_to_bytes::base64_to_bytes;
use cryptopals::pkcs7::pkcs7_unpad;
use cryptopals::xor_bytes::xor_bytes_const;
use tap::Pipe as _;
use tap::Tap as _;

#[test]
fn solution() -> anyhow::Result<()> {
  let input = include_str!("set2_challenge10_input.txt")
    .lines()
    .collect::<String>()
    .pipe_as_ref(base64_to_bytes)?;

  let output = aes_128_cbc_decrypt(*b"YELLOW SUBMARINE", [0; 16], &input)?
    .pipe(String::from_utf8)
    .map_err(|err| anyhow::anyhow!(err))?;

  assert_eq!(&output, include_str!("set2_challenge10_output.txt"));

  Ok(())
}
fn aes_128_cbc_decrypt(
  key: [u8; 16],
  iv: [u8; 16],
  input: &[u8],
) -> anyhow::Result<Vec<u8>> {
  let aes_128 = Aes128::new(GenericArray::from_slice(&key));

  input
    .array_chunks::<16>()
    .copied()
    .scan(iv, |previous_input_block, input_block| {
      let decrypted_input_block =
        input_block.clone().tap_borrow_mut(|mut input_block| {
          aes_128.decrypt_block(GenericArray::from_mut_slice(&mut input_block));
        });

      let output_block =
        xor_bytes_const(*previous_input_block, decrypted_input_block);

      *previous_input_block = input_block;

      Some(output_block)
    })
    .flatten()
    .collect::<Vec<u8>>()
    .pipe(pkcs7_unpad)
}

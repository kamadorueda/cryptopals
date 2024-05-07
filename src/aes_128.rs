use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockDecrypt as _;
use aes::cipher::KeyInit as _;
use aes::Aes128;
use cipher::block_padding::Pkcs7;
use tap::Pipe;

pub fn aes_128_decrypt(key: [u8; 16], input: &[u8]) -> anyhow::Result<Vec<u8>> {
  key
    .pipe_as_ref(GenericArray::from_slice)
    .pipe(Aes128::new)
    .decrypt_padded_vec::<Pkcs7>(input)
    .map_err(|err| anyhow::anyhow!(err))
}

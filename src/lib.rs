pub mod rsatools;
pub mod aestools;
mod misctools;
mod matrix;

use rand::random;
use rsa::RsaPrivateKey;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use crate::aestools::{decrypt, encrypt, schedule, CryptError, SBOX};
use crate::misctools::{get_chunks, pad, CryptTools};

const KEY_BYTES: usize = 16;

pub fn generate_key() -> Vec<u8> {
  let mut retval = [0u8; KEY_BYTES];
  
  for i in 0..KEY_BYTES {
    retval[i] = random();
  }
  
  return retval.to_vec();
}

#[test]
pub fn test_aes() -> Result<(), CryptError> {
  let key = generate_key();
  
  let text = "The gold is hidden under the back left corner of my room.".to_string();
  let mut raw_data = text.clone().into_bytes();
  
  println!("before encryption: \"{}\"", text);
  
  let encrypted = encrypt(&key, raw_data).unwrap_or_else(|e| {
    println!("{:x?}", e);
    vec![]
  });
  
  let intermediate = unsafe {
    String::from_utf8_unchecked(encrypted.clone())
  };
  
  println!("intermediate: \"{}\"", intermediate);

  let decrypted = decrypt(&key, encrypted).unwrap_or_else(|e| {
    println!("{:x?}", e);
    vec![]
  });
  
  let post_decrypt = {
    let plaintext = match String::from_utf8(decrypted) {
      Ok(str) => str,
      Err(e) => return Err(CryptError::Other(format!("{:?}", e)))
    };
    
    plaintext.trim_end().to_string()
  };
  
  println!("after decryption: \"{}\"", post_decrypt);
  
  assert_eq!(text, post_decrypt);
  
  Ok(())
}

#[test]
pub fn test_rsa() {
  let mut rng = rand::thread_rng();
  let rsa_key = RsaPrivateKey::new(&mut rng, 1024).expect("couldn't generate keys!");
  
  let e = rsa_key.e().to_bytes_be();
  let n = rsa_key.n().to_bytes_be();
  let d = rsa_key.d().to_bytes_be();
  
  let aes_key = generate_key();
  let ciphertext = rsatools::encrypt_key(&aes_key, &e, &n);
  let reconstructed = rsatools::decrpyt_key(&ciphertext, &d, &n);
  
  assert_eq!(aes_key.to_vec(), reconstructed)
}
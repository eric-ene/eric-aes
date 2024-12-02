use rsa::BigUint;

pub fn encrypt_key(key: &Vec<u8>, e: &Vec<u8>, n: &Vec<u8>) -> Vec<u8> {
  let e = BigUint::from_bytes_be(&e);
  let n = BigUint::from_bytes_be(&n);

  let m = BigUint::from_bytes_be(&key);

  let c = m.modpow(&e, &n);
  
  return c.to_bytes_be();
}

pub fn decrpyt_key(ciphertext: &Vec<u8>, d: &Vec<u8>, n: &Vec<u8>) -> Vec<u8> {
  let d = BigUint::from_bytes_be(&d);
  let n = BigUint::from_bytes_be(&n);

  let c = BigUint::from_bytes_be(&ciphertext);
  
  let m = c.modpow(&d, &n);

  return m.to_bytes_be();
}
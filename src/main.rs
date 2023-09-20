extern crate aes_gcm;
extern crate rand;

use std::io::{Error, ErrorKind};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng,generic_array::GenericArray,Payload},
    Aes256Gcm, Key, Nonce
};

use rand::Rng;


pub struct block {
    data: Vec<u8>,
    nonce: Vec<u8>
}
fn main() {
   let enc = encrypt("soloworld".as_bytes(), "5zOYqRvMSbbNPRfFtct3fYogQszzucF7");
}


pub fn decrypt( encryptedTex: &block,  password:&str) {
    let password_byte = password.as_bytes();
    let key: &Key<Aes256Gcm> = password_byte.into();
    let nonce = &encryptedTex.nonce;
    let data = &encryptedTex.data;

   let nonce = aes_gcm::Nonce::from_slice(&nonce);


    let cipher = Aes256Gcm::new(&key);
    let op = cipher.decrypt(&nonce ,data.as_slice());
    //  let plaintext = cipher.decrypt(nonce, &*encryptedTex[15..].as_ref()).unwrap();
}
pub fn encrypt(data: &[u8], password: &str)  {
    let password_byte = password.as_bytes();

    let key: &Key<Aes256Gcm> = password_byte.into();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);


    let encrypted_data = match cipher.encrypt(&nonce,data)
    {
        Ok(encrpted) => {

            let e = block { data: encrpted, nonce:nonce.to_vec() };
            e
        }
        Err(err) => {
            panic!("could not encrypt")
        }
    };


decrypt(&encrypted_data ,password);


}

extern crate aes_gcm;
extern crate rand;

use std::fs::{File, OpenOptions};
use std::io::{ ErrorKind, Read, Write};
use std::path::Path;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng,generic_array::GenericArray,Payload},
    Aes256Gcm, Key, Nonce
};
use std::env;
use rand::Rng;


pub struct block {
    data: Vec<u8>,
    nonce: Vec<u8>
}

 static  KEY:&str = "5zOYqRvMSbbNPRfFtct3fYogQszzucF7";
fn main() {




    let mut  open_file_read = File::options().read(true).open("/Users/hokage/Documents/1.jpeg",).expect("couldnot open file");
    let mut buff = Vec::new();
    let input=  open_file_read.read_to_end(&mut buff).expect("coould not read files");
    println!("{:?}", buff);
    println!("..............printing nonce .................");
}


pub fn decrypt( encrypted_tex: &block,  password:&str) {
    let password_byte = password.as_bytes();
    let key: &Key<Aes256Gcm> = password_byte.into();
    let nonce = &encrypted_tex.nonce;
    let data = &encrypted_tex.data;

   let nonce = aes_gcm::Nonce::from_slice(&nonce);


    let cipher = Aes256Gcm::new(&key);
    let op = cipher.decrypt(&nonce ,data.as_slice());
}
pub fn encrypt(data: &[u8], password: &str)  -> Result<block, aes_gcm::Error>  {
    let password_byte = password.as_bytes();

    let key: &Key<Aes256Gcm> = password_byte.into();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);


    let encrypted_data = match cipher.encrypt(&nonce,data)
    {
        Ok(encrpted) => {

            let e = block { data: encrpted, nonce:nonce.to_vec() };
           return  Ok(e)
        }
        Err(err) => {
           return Err(err) ;
        }
    };    }





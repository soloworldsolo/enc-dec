extern crate aes_gcm;
extern crate rand;


use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use chrono::NaiveDate;

pub struct EncryptedBlock {
    data: Vec<u8>,
    nonce: Vec<u8>,
}

static KEY: &str = "5zOYqRvMSbbNPRfFtct3fYogQszzucF7";

fn main() {
    read_file_encrypt("/Users/hokage/Documents/one.jpeg","/Users/hokage/Documents/kick_new.enc");
    decrypt_file("/Users/hokage/Documents/kick_new.enc" ,"/Users/hokage/Documents/one_decrypted.jpeg");
}

pub fn decrypt_file(input_path: &str ,output_path:&str) {
    let mut buff = read_file_from_path(input_path);
    let encrypted_block = split_encrypt_block(buff);
    let decrypted_chunks = decrypt(&encrypted_block ,KEY).expect("could not decrypt chunka");
    write_file_to_path(output_path ,decrypted_chunks);
}

pub fn split_encrypt_block(buff: Vec<u8>) -> EncryptedBlock {
    let mut nonce: Vec<u8> = vec![];
    let mut data: Vec<u8> = vec![];
    let breaking_point = 12;
    let mut currentIndex = 0;
    for current_byte in buff {
        if (currentIndex < breaking_point) {
            nonce.push(current_byte);
        } else {
            data.push(current_byte);
        }
        currentIndex += 1;

    }
    return EncryptedBlock {
        nonce,
        data,
    };
}

pub fn decrypt(encrypted_tex: &EncryptedBlock, password: &str) -> Result<Vec<u8>,aes_gcm::Error>  {
    let password_byte = password.as_bytes();
    let key: &Key<Aes256Gcm> = password_byte.into();
    let nonce = &encrypted_tex.nonce;
    let data = &encrypted_tex.data;
    let nonce = aes_gcm::Nonce::from_slice(&nonce);


    let cipher = Aes256Gcm::new(&key);
    let op = cipher.decrypt(&nonce, data.as_slice());
    return op;
}

pub fn encrypt(data: &[u8], password: &str) -> Result<EncryptedBlock, aes_gcm::Error> {
    let password_byte = password.as_bytes();

    let key: &Key<Aes256Gcm> = password_byte.into();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);


    let encrypted_data = match cipher.encrypt(&nonce, data)
    {
        Ok(encrypted) => {
            let e = EncryptedBlock { data: encrypted, nonce: nonce.to_vec() };
            return Ok(e);
        }
        Err(err) => {
            return Err(err);
        }
    };
}


pub fn read_file_encrypt(input_path: &str, output_path:&str) {
    let binding = read_file_from_path(input_path);
    let buff = binding.as_slice();
    let encrypted_text = encrypt(&buff, KEY);

    match encrypted_text {
        Ok(res) => {
            let mut input_bytes = Vec::new();
            let mut nonce = res.nonce;
            let mut data = res.data;
            input_bytes.append(&mut nonce);
            input_bytes.append(&mut data);
            write_file_to_path(output_path,input_bytes);
        }
        Err(err) => {
            print!("{:?}", err);
        }
    }
}

pub fn read_file_from_path(path: &str) -> Vec<u8> {
    let mut open_file_read = File::options().read(true).open(path).expect("couldnot open file");
    let mut input_holder = Vec::new();
    let input_data = open_file_read.read_to_end(&mut input_holder).expect("could not read files");
    return input_holder;
}


pub  fn write_file_to_path(input_path:&str, bytes:Vec<u8>)  {
    let path = Path::new(input_path);
    let mut write_path = File::options().
        create(true)
        .append(true).
        open(path)
        .expect("could not create file");
    let mut writer = BufWriter::new(write_path);
    writer.write_all(bytes.as_slice()).expect("couldnot write in to buffer");
}


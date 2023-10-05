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
    read_file_encrypt("C:\\Users\\Windows\\OneDrive\\Pictures\\evv.png");
    println!("..............printing nonce .................");
}


pub fn decrypt(encrypted_tex: &EncryptedBlock, password: &str) {
    let password_byte = password.as_bytes();
    let key: &Key<Aes256Gcm> = password_byte.into();
    let nonce = &encrypted_tex.nonce;
    let data = &encrypted_tex.data;

    let nonce = aes_gcm::Nonce::from_slice(&nonce);


    let cipher = Aes256Gcm::new(&key);
    let op = cipher.decrypt(&nonce, data.as_slice());
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


pub fn read_file_encrypt(input_path: &str) {
    let mut open_file_read = File::options().read(true).open(input_path).expect("couldnot open file");
    let mut buff = Vec::new();
    let input_data = open_file_read.read_to_end(&mut buff).expect("could not read files");
    let encrypted_text = encrypt(&buff, KEY);

    match encrypted_text {
        Ok(res) => {
            let mut input_bytes = Vec::new();
            let mut nonce = res.nonce;
            input_bytes.append(&mut nonce);

            let path = Path::new(input_path);
            let mut parent = path.parent().expect("could not expect path");
            let mut new_filename = String::new();
            let dt = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(0, 0, 1, 444).unwrap().and_local_timezone(chrono::Utc).unwrap();
            new_filename.push_str(parent.to_str().unwrap());
            new_filename.push_str("\\");
            new_filename.push_str(&*dt.timestamp_millis().to_string());
            new_filename.push_str(".enc");
            println!("{:?}", &new_filename);
            let mut data = res.data;
            input_bytes.append(&mut data);
            let mut write_path = File::options().
                create(true)
                .append(true).
                open(new_filename)
                .expect("could not create file");
            let mut writer = BufWriter::new(write_path);
            writer.write_all(input_bytes.as_slice()).expect("couldnot write in to buffer");
        }
        Err(err) => {
            print!("{:?}", err);
        }
    }
}


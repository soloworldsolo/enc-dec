use std::fs;
use std::path::Path;
use pbkdf2::Pbkdf2;
use sha2::Sha256;


fn main() {
    println!("Hello, world!");
    let path = Path::new("/Users/hokage/Documents/edited.jpeg");
    let vecl = fs::read(&path).expect("could not re");
    let  key = b"key-generatorreanddddom";
    let mut buf = [0u8; 20];
    let  vec_arr = vecl.as_slice();
    let salt = b"salsalesalasale";
    println!("{:?}",vec_arr);
    let mut key1 = [0u8; 20];

    pbkdf2::pbkdf2_hmac::<Sha256>(vec_arr,salt,500,&mut key1);

    println!("{:?}", vec_arr)
}

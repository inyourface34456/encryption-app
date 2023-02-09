#[macro_use] extern crate rocket;
use base64::{Engine as _, engine::general_purpose};
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;
use rocket::{fs::NamedFile, get};

#[get("/encrypt")]
async fn brow_entry() -> Result<NamedFile, std::io::Error> {
   NamedFile::open("static/index.html").await
}

#[post("/encrypt", data="<private>")]
async fn encrypt(private: &str) -> String {
    let keypair = Rsa::generate(2048).unwrap();
    
    let keypair = PKey::from_rsa(keypair).unwrap();
    let data = private.as_bytes();

    // Encrypt the data with RSA PKCS1
    let mut encrypter = Encrypter::new(&keypair).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(data).unwrap();
    let mut encrypted = vec![0; buffer_len];
    // Encrypt and truncate the buffer
    let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
    encrypted.truncate(encrypted_len);
    // let s = String::from_utf8_lossy(&encrypted);
    general_purpose::URL_SAFE_NO_PAD.encode(&encrypted)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, encrypt, brow_entry])
}
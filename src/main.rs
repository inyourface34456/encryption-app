#[macro_use] extern crate rocket;
mod id_sys;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use rocket::response::content::{RawText, RawHtml};
use openssl::pkey::PKey;
use rocket::tokio::fs::File;
use rocket::get;

// https://api.rocket.rs/v0.5-rc/rocket/http/struct.ContentType.html
/*
todo:
get the private key as a pkcs8
get the public key from pkcs8
write them to two seprate files
show them to the user as a download (not copy and paste)
find out how to zip them together as one download
*/
#[post("/encrypt", data="<private>")]
async fn encrypt(private: &str) -> String {
    let private: Vec<&str> = private.split("{delimanator}").collect();
    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();
    let data = private[0].as_bytes();
    
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}priv.key", private[0])).expect("create failed");
    file.write_all(String::from_utf8_lossy(&keypair.private_key_to_pem_pkcs8().unwrap()).as_bytes()).expect("write failed");
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}pub.key", private[0])).expect("create failed");
    file.write_all(String::from_utf8_lossy(&keypair.public_key_to_pem().unwrap()).as_bytes()).expect("write failed");
    
    // Encrypt the data with RSA PKCS1
    let mut encrypter = Encrypter::new(&keypair).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
    // Create an output buffer
    let buffer_len = encrypter.encrypt_len(data).unwrap();
    let mut encrypted = vec![0; buffer_len];
    // Encrypt and truncate the buffer
    let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap(); 
    encrypted.truncate(encrypted_len);
    // let s = String::from_utf8_lossy(&encrypted);\
    general_purpose::URL_SAFE_NO_PAD.encode(&encrypted)
}

#[get("/<folder_name>")]
async fn index(folder_name: String) -> Option<RawHtml<File>> {
    File::open(format!("/workspaces/codespaces-blank/encryption-app/src/static/{}/index.html", folder_name)).await.map(RawHtml).ok()
}
#[get("/<folder_name>/<file_name>")]
async fn get_sheets(folder_name: String, file_name: String) -> Option<RawText<File>> {
    File::open(format!("/workspaces/codespaces-blank/encryption-app/src/static/{}/{}", folder_name, file_name)).await.map(RawText).ok()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, encrypt, get_sheets])
}
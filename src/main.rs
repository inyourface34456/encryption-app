#[macro_use] extern crate rocket;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};
use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use rocket::response::content::{RawText, RawHtml};
// use openssl::pkey::{PKey, Private};
use rocket::tokio::fs::File;
use rocket::get;

const DLIM: &str = "{delimanator}";

#[post("/encrypt", data="<private>")]
async fn encrypt(private: &str) -> String {
    let private: Vec<&str> = private.split(DLIM).collect();
    let passphrase = private[1];
    let data = private[0];

    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}priv.key", passphrase)).expect("create failed");
    file.write_all(String::from_utf8_lossy(&private_key).as_bytes()).expect("write failed");
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}pub.key", passphrase)).expect("create failed");
    file.write_all(String::from_utf8_lossy(&public_key).as_bytes()).expect("write failed");

    let rsa = Rsa::public_key_from_pem(String::from_utf8_lossy(&public_key).as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();

    general_purpose::URL_SAFE_NO_PAD.encode(String::from_utf8_lossy(&buf).to_string())
}

#[post("/decrypt", data="<data>")]
async fn decrypt(data: String) -> String {
    let data: Vec<&str> = data.split(DLIM).collect();
    let passphrase = data[1];
    let raw_data = data[0];
    println!("{}", passphrase);
    let private_key_pem = std::fs::read(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}priv.key", passphrase)).unwrap();

    let rsa = Rsa::private_key_from_pem_passphrase(String::from_utf8_lossy(&private_key_pem).as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(general_purpose::URL_SAFE_NO_PAD.decode(raw_data).unwrap().as_slice(), &mut buf, Padding::PKCS1).unwrap();

    String::from_utf8(buf).unwrap()
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
    rocket::build().mount("/", routes![index, encrypt, get_sheets, decrypt])
}
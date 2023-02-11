#[macro_use] extern crate rocket;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};
use openssl::encrypt::Encrypter;
use openssl::rsa::{Rsa, Padding};
use rocket::response::content::{RawText, RawHtml};
use openssl::pkey::{PKey, Private};
use rocket::tokio::fs::File;
use rocket::get;

const DLIM: &str = "{delimanator}";

#[post("/encrypt", data="<private>")]
async fn encrypt(private: &str) -> String {
    let private: Vec<&str> = private.split(DLIM).collect();
    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();
    let data = private[0].as_bytes();
    
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}priv.key", private[1])).expect("create failed");
    file.write_all(String::from_utf8_lossy(&keypair.private_key_to_pem_pkcs8().unwrap()).as_bytes()).expect("write failed");
    let mut file = std::fs::File::create(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}pub.key", private[1])).expect("create failed");
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

#[post("/decrypt", data="<data>")]
async fn decrypt(data: String) -> String {
    // from: https://stackoverflow.com/questions/58426023/encrypt-decrypt-large-text-using-rusts-openssl-library
    let data: Vec<&str> = data.split(DLIM).collect();
    let priv_key = std::fs::read(format!("/workspaces/codespaces-blank/encryption-app/src/keys/{}priv.key", data[1])).unwrap();
    let enc_data = data[0].as_bytes();
    let data_len = enc_data.len();
    let private_rsa: Rsa<Private> = Rsa::private_key_from_pem(priv_key.as_slice()).unwrap();
    let buf_len = private_rsa.size() as usize;
    let mut buffer: Vec<u8> = vec![0; buf_len];
    let mut decrypted_data: Vec<u8> = vec![0; data_len];
    for chunk in enc_data.chunks(buf_len) {
        private_rsa.private_decrypt(chunk, &mut buffer, Padding::NONE).expect("Error Decrypting");
        decrypted_data.extend_from_slice(buffer.as_slice());
    }
    String::from_utf8_lossy(&decrypted_data).to_string()
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
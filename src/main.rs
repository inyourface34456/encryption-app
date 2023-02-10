#[macro_use] extern crate rocket;
use rocket::http::ContentType;
use base64::{Engine as _, engine::general_purpose};
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use rocket::response::content::RawText;
use openssl::pkey::PKey;
use rocket::tokio::fs::File;
use rocket::{fs::NamedFile, get};

// https://api.rocket.rs/v0.5-rc/rocket/http/struct.ContentType.html
#[get("/encrypt")]
async fn brow_entry() -> Result<NamedFile, std::io::Error> {
   NamedFile::open("/workspaces/codespaces-blank/encryption-app/src/static/encrypt/index.html").await
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
#[get("/<folder_name>/<file_name>")]
async fn get_sheets(folder_name: String, file_name: String) -> Option<RawText<File>> {
    File::open(format!("/workspaces/codespaces-blank/encryption-app/src/static/{}/{}", folder_name, file_name)).await.map(RawText).ok()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, encrypt, brow_entry, get_sheets])
}
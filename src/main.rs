#[macro_use] extern crate rocket;
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;
use rocket::{fs::NamedFile, get};

#[get("/encrypt")]
async fn brow_entry() -> Result<NamedFile, std::io::Error> {
   NamedFile::open("static/index.html").await
}

#[post("/encrypt", data="<private>")]
async fn encrypt(private: Vec<u8>) -> Vec<u8> {

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, encrypt, brow_entry])
}
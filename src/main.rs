#[macro_use] extern crate rocket;
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;
use rocket::{fs::NamedFile, get};
use std::fs;

#[get("/encrypt")]
async fn encrypt() -> Result<NamedFile, std::io::Error> {
   NamedFile::open("static/index.html").await
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, encrypt])
}
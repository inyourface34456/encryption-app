#[macro_use] extern crate rocket;
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
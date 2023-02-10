// this is all from the rocket example for a cheep pastebin clone.
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use rocket::request::FromParam;

/// A _probably_ unique paste ID.
#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(pass: String) -> PasteId<'static> {
        let id = String::from(pass);        
        PasteId(Cow::Owned(id))
    }

    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = "/workspaces/codespaces-blank/encryption-app/src/keys";
        Path::new(root).join(self.0.as_ref())
    }
}


impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}
use crate::models::model::Model;
use anyhow::{Error, Result};
use futures::stream::BoxStream;

pub struct Gemini<'a> {
    token: &'a str,
}

impl<'a> Gemini<'a> {
    pub fn new(token: &'a str) -> Gemini<'a> {
        Gemini { token }
    }
}

impl<'a> Model for Gemini<'a> {
    fn call(&self, _prompt: &str) -> Result<BoxStream<'static, String>> {
        println!("Calling on Gemini");
        Err(Error::msg("not implemented"))
    }

    fn name(&self) -> &str {
        "Gemini"
    }
}
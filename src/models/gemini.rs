use crate::models::model::Model;
use anyhow::{Error, Result};
use futures::stream::BoxStream;

pub struct Gemini {}

impl Gemini {
    pub fn new() -> Gemini {
        Gemini {}
    }
}

impl<'a> Model<'a> for Gemini {
    fn call(&self, _prompt: &str) -> Result<BoxStream<'a, String>> {
        println!("Calling on Gemini");
        Err(Error::msg("not implemented"))
    }

    fn name(&self) -> &str {
        "Gemini"
    }
}

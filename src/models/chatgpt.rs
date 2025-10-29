use crate::models::model::Model;
use anyhow::{Error, Result};
use futures::stream::BoxStream;

pub struct ChatGPT {
}

impl ChatGPT {
    pub fn new() -> ChatGPT {
        ChatGPT { }
    }
}

impl<'a> Model<'a> for ChatGPT {
    fn call(&self, _prompt: &str) -> Result<BoxStream<'a, String>> {
        println!("Calling on ChatGPT");
        Err(Error::msg("not implemented"))
    }

    fn name(&self) -> &str {
        "ChatGPT"
    }
}
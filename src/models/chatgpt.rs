use crate::models::model::Model;
use anyhow::{Error, Result};
use futures::stream::BoxStream;

pub struct ChatGPT<'a> {
    token: &'a str,
}

impl<'a> ChatGPT<'a> {
    pub fn new(token: &'a str) -> ChatGPT<'a> {
        ChatGPT { token }
    }
}

impl<'a> Model for ChatGPT<'a> {
    fn call(&self, _prompt: &str) -> Result<BoxStream<'static, String>> {
        println!("Calling on ChatGPT");
        Err(Error::msg("not implemented"))
    }

    fn name(&self) -> &str {
        "ChatGPT"
    }
}
use anyhow::Result;
use futures::stream::BoxStream;
use rand::random;

use crate::models::Claude;
use crate::models::Gemini;
use crate::models::ChatGPT;

pub trait Model {
    fn call(&self, prompt: &str) -> Result<BoxStream<'static, String>>;
    fn name(&self) -> &str;
}

pub fn select() -> Box<dyn Model + Send> {
    match random::<u32>() % 3 {
        0 => Box::new(Claude::new("whatever")),
        1 => Box::new(ChatGPT::new("whatever")),
        2 => Box::new(Gemini::new("whatever")),
        _ => panic!("How the hell did you get here?"),
    }
}
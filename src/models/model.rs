use anyhow::Result;
use futures::stream::BoxStream;
use rand::random;

use crate::models::ChatGPT;
use crate::models::Claude;
use crate::models::Gemini;

pub trait Model<'a> {
    fn call(&self, prompt: &str) -> Result<BoxStream<'a, String>>;
    fn name(&self) -> &str;
    fn max_output(&self) -> i32 {
        1024 
    }
}

pub fn select<'a>() -> Box<dyn Model<'a> + Send> {
    match random::<u32>() % 3 {
        0 => Box::new(Claude::new()),
        1 => Box::new(ChatGPT::new()),
        2 => Box::new(Gemini::new()),
        _ => panic!("How the hell did you get here?"),
    }
}

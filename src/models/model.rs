use anyhow::Result;
use futures::stream::{BoxStream, StreamExt};
use rand::random;
use reqwest::header::HeaderMap;
use futures::future::Ready;
use reqwest_eventsource::{retry::Never, Error as RESError, Event, RequestBuilderExt};
use reqwest;


use crate::models::ChatGPT;
use crate::models::Claude;
use crate::models::Gemini;

pub trait Model<'a> {
    fn name(&self) -> &str;

    fn url(&self) -> &str;
    fn headers(&self) -> HeaderMap;
    fn body(&self, prompt: &str) -> String;
    fn event_handler(&self) -> fn((String, String)) -> Ready<Option<String>>;

    fn max_output(&self) -> i32 {
        1024 
    }

    fn call(&self, prompt: &str) -> Result<BoxStream<'a, String>> {
        let client = reqwest::Client::new();
        let mut response = client
            .post(self.url())
            .headers(self.headers())
            .header("content-type", "application/json")
            .body(self.body(prompt))
            .eventsource()?;

        response.set_retry_policy(Box::new(Never {}));
        Ok(response
            .filter_map(|event| async move {
                match event {
                    Ok(Event::Open) => None,
                    Ok(Event::Message(msg)) => Some((msg.event, msg.data)),
                    Err(RESError::StreamEnded) => None,
                    Err(e) => {
                        eprintln!("{e:?}");
                        None
                    }
                }
            })
            .filter_map(self.event_handler())
            .boxed())
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

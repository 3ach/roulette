use crate::models::model::Model;

use anyhow::Result;
use futures::stream::{BoxStream, StreamExt};
use json;
use reqwest_eventsource::{retry::Never, Error as RESError, Event, RequestBuilderExt};
use reqwest;

pub struct ChatGPT {}

impl ChatGPT {
    pub fn new() -> ChatGPT {
        ChatGPT {}
    }
}


impl<'a> Model<'a> for ChatGPT {
    fn call(&self, prompt: &str) -> Result<BoxStream<'a, String>> {
        let mut body = json::JsonValue::new_object();
        body["model"] = "gpt-4.1".into();
        body["max_output_tokens"] = self.max_output().into();
        body["input"] = prompt.into();
        body["stream"] = true.into();

        let client = reqwest::Client::new();
        let mut response = client
            .post("https://api.openai.com/v1/responses")
            .bearer_auth(env!("OPENAI_API_KEY"))
            .header("content-type", "application/json")
            .body(json::stringify(body))
            .eventsource()?;

        response.set_retry_policy(Box::new(Never {}));
        Ok(response
            .filter_map(|event| async move {
                match event {
                    Ok(Event::Open) => None,
                    Ok(Event::Message(msg)) => Some(msg),
                    Err(RESError::StreamEnded) => None,
                    Err(e) => {
                        eprintln!("{e:?}");
                        None
                    }
                }
            })
            .filter_map(|event| async move {
                match event.event.as_str() {
                    "response.failed" => {
                        eprintln!("Error event: {:?}", event.data);
                        None
                    }
                    "response.output_text.delta" => Some(event.data),
                    _ => None,
                }
            })
            .map(|data| {
                json::parse(&data)
                    .map(|d| d["delta"].to_string())
                    .unwrap_or(String::new())
            })
            .boxed())
    }

    fn name(&self) -> &str {
        "ChatGPT"
    }
}

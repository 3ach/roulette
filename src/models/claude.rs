use crate::models::model::Model;

use json;
use reqwest::header::HeaderMap;
use std::env::var;
use futures::future::{Ready, ready};

pub struct Claude {}

impl Claude {
    pub fn new() -> Claude {
        Claude {}
    }
}

impl<'a> Model<'a> for Claude {
    fn url(&self) -> &str {
        "https://api.anthropic.com/v1/messages"
    }
    
    fn headers(&self) -> HeaderMap {
        let token = var("ANTHROPIC_API_KEY").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", token.parse().unwrap());
        headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
        headers
    }

    fn body(&self, prompt: &str) -> String {
        let mut message = json::JsonValue::new_object();
        message["role"] = "user".into();
        message["content"] = prompt.into();

        let mut messages = json::JsonValue::new_array();
        messages.push(message).unwrap();

        let mut body = json::JsonValue::new_object();
        body["model"] = "claude-sonnet-4-5".into();
        body["max_tokens"] = self.max_output().into();
        body["stream"] = true.into();
        body["messages"] = messages;

        json::stringify(body)
    }

    fn event_handler(&self) -> fn((String, String)) -> Ready<Option<String>> {
        |(event, data)| {
            let result = if event == "error" {
                eprintln!("Failed: {data}");
                None
            } else if event == "content_block_delta" {
                json::parse(&data)
                    .map(|d| d["delta"]["text"].to_string())
                    .ok()
            } else {
                None
            };

            ready(result)
        }
    }

    fn name(&self) -> &str {
        "Claude"
    }
}

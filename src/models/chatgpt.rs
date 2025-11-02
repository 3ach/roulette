use crate::models::model::Model;

use json;
use reqwest::header::HeaderMap;
use std::env::var;
use futures::future::{Ready, ready};

pub struct ChatGPT {}

impl ChatGPT {
    pub fn new() -> ChatGPT {
        ChatGPT {}
    }
}


impl<'a> Model<'a> for ChatGPT {
    fn url(&self) -> &str {
        "https://api.openai.com/v1/responses"
    }

    fn headers(&self) -> HeaderMap {
        let token = var("OPENAI_API_KEY").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        headers
    }

    fn body(&self, prompt: &str) -> String {
        let mut body = json::JsonValue::new_object();
        body["model"] = "gpt-4.1".into();
        body["max_output_tokens"] = self.max_output().into();
        body["input"] = prompt.into();
        body["stream"] = true.into();

        json::stringify(body)
    }

    fn event_handler(&self) -> fn((String, String)) -> Ready<Option<String>> {
        |(event, data)| {
            let result = if event == "response.failed" {
                eprintln!("Failed: {data}");
                None 
            } else if event == "response.output_text.delta" {
                json::parse(&data)
                    .map(|d| d["delta"].to_string())
                    .ok()
            } else {
                None
            };

            ready(result)
        }
    }

    fn name(&self) -> &str {
        "ChatGPT"
    }
}

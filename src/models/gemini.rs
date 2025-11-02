use crate::models::model::Model;

use json;
use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::env::var;
use futures::future::{ready, Ready};

pub struct Gemini {}

impl Gemini {
    pub fn new() -> Gemini {
        Gemini {}
    }
}

impl<'a> Model<'a> for Gemini {
    fn url(&self) -> &str {
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:streamGenerateContent?alt=sse"
    }

    fn headers(&self) -> HeaderMap {
        let token = var("GEMINI_API_KEY").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("x-goog-api-key", token.parse().unwrap());
        headers
    }

    fn body(&self, prompt: &str) -> String {
        let mut parts = json::JsonValue::new_array();
        parts.push(HashMap::from([("text", prompt)])).unwrap();

        let mut contents = json::JsonValue::new_array();
        contents.push(HashMap::from([("parts", parts)])).unwrap();

        let mut generation_config = json::JsonValue::new_object();
        generation_config["maxOutputTokens"] = self.max_output().into();
        generation_config["thinkingConfig"] = HashMap::from([("thinkingBudget", 0)]).into();

        let mut body = json::JsonValue::new_object();
        body["contents"] = contents;
        body["generationConfig"] = generation_config;

        json::stringify(body)
    }

    fn event_handler(&self) -> fn((String, String)) -> Ready<Option<String>> {
        |(event, data)| {
            let result = if event != "message" {
                None
            } else {
                json::parse(&data)
                    .map(|d| d["candidates"][0]["content"]["parts"][0]["text"].to_string())
                    .ok()
            };

            ready(result)
        }
    }

    fn name(&self) -> &str {
        "Gemini"
    }
}

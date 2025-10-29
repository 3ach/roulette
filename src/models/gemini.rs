use crate::models::model::Model;

use anyhow::Result;
use futures::stream::{BoxStream, StreamExt};
use json;
use reqwest_eventsource::{retry::Never, Error as RESError, Event, RequestBuilderExt};
use std::collections::HashMap;
use reqwest;

pub struct Gemini {}

impl Gemini {
    pub fn new() -> Gemini {
        Gemini {}
    }
}
/*
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:streamGenerateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  --no-buffer \
  -d '{
    "contents": [
      {
        "parts": [
          {
            "text": "Explain how AI works"
          }
        ]
      }
    ]
  }'
*/

impl<'a> Model<'a> for Gemini {
    fn call(&self, prompt: &str) -> Result<BoxStream<'a, String>> {
        let mut parts = json::JsonValue::new_array();
        parts.push(HashMap::from([("text", prompt)]))?;

        let mut contents = json::JsonValue::new_array();
        contents.push(HashMap::from([("parts", parts)]))?;

        let mut generation_config = json::JsonValue::new_object();
        generation_config["maxOutputTokens"] = self.max_output().into();
        generation_config["thinkingConfig"] = HashMap::from([("thinkingBudget", 0)]).into();

        let mut body = json::JsonValue::new_object();
        body["contents"] = contents;
        body["generationConfig"] = generation_config;

        let client = reqwest::Client::new();
        let mut response = client
            .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:streamGenerateContent?alt=sse")
            .header("x-goog-api-key", env!("GEMINI_API_KEY"))
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
                    "message" => Some(event.data),
                    _ => None,
                }
            })
            .map(|data| {
                json::parse(&data)
                    .map(|d| d["candidates"][0]["content"]["parts"][0]["text"].to_string())
                    .unwrap_or(String::new())
            })
            .boxed())
    }

    fn name(&self) -> &str {
        "Gemini"
    }
}

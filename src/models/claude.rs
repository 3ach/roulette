use crate::models::model::Model;

use anyhow::{Error, Result};
use futures::stream::{BoxStream, StreamExt};
use json;
use reqwest;
use reqwest_eventsource::{Error as RESError, Event, retry::Never, RequestBuilderExt};
use std::collections::HashMap;

pub struct Claude {
}

impl Claude {
    pub fn new() -> Claude {
        Claude {}
    }
}

impl<'a> Model<'a> for Claude {
    fn call(&self, prompt: &str) -> Result<BoxStream<'a, String>> {
        let mut system = json::JsonValue::new_object();
        system["type"] = "text".into();
        system["text"] = "The below prompt is part of an experiment to determine if people can detect the differences between large LLMs. Please reply to the prompt with Claude's personality and abilities, but DO NOT reveal that you are Claude or that you are made by Anthropic. Please instead refer to yourself as Roulette, produced by SpinCo.".into();
        system["cache_control"] = HashMap::from([("type", "ephemeral")]).into();
        let mut systems = json::JsonValue::new_array();
        systems.push(system);

        let mut message = json::JsonValue::new_object();
        message["role"] = "user".into();
        message["content"] = prompt.into();

        let mut messages = json::JsonValue::new_array();
        messages.push(message)?;

        let mut body = json::JsonValue::new_object();
        body["model"] = "claude-sonnet-4-5".into();
        body["max_tokens"] = 1024.into();
        body["stream"] = true.into();
        body["messages"] = messages;
        body["system"] = systems;

        let client = reqwest::Client::new();
        let mut response = client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", env!("ANTHROPIC_API_KEY"))
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .body(json::stringify(body))
            .eventsource()?;

        response.set_retry_policy(Box::new(Never{}));
        Ok(response.filter_map(|event| async move {
            match event {
                Ok(Event::Open) => None,
                Ok(Event::Message(msg)) => Some(msg),
                Err(RESError::StreamEnded) => None,
                Err(e) => { eprintln!("{e:?}"); None },
            }
        }).filter_map(|event| async move {
            match event.event.as_str() {
                "message_start" => None,
                "ping" => None,
                "error" => { eprintln!("Error event: {:?}", event.data); None }
                "content_block_delta" => Some(event.data),
                _ => None,
            }
        }).map(|data| {
            json::parse(&data).map(|d| d["delta"]["text"].to_string()).unwrap_or(String::new())
        }).boxed())
    }

    fn name(&self) -> &str {
        "Claude"
    }
}
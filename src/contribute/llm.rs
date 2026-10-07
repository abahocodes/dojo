//! Minimal clients for drafting questions with structured (JSON-schema)
//! output: the Anthropic Messages API and OpenAI Chat Completions, over raw
//! HTTP (there is no official Rust SDK).

use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Claude,
    OpenAi,
    /// A local model through Ollama: free, nothing leaves the machine.
    Ollama,
}

impl Provider {
    pub fn parse(s: &str) -> Option<Provider> {
        match s.to_ascii_lowercase().as_str() {
            "claude" | "anthropic" => Some(Provider::Claude),
            "openai" | "gpt" => Some(Provider::OpenAi),
            "ollama" | "local" => Some(Provider::Ollama),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Provider::Claude => "claude",
            Provider::OpenAi => "openai",
            Provider::Ollama => "ollama",
        }
    }

    /// Whether drafting needs an API key (local models don't).
    pub fn needs_key(self) -> bool {
        self != Provider::Ollama
    }

    pub fn label(self) -> &'static str {
        match self {
            Provider::Claude => "Anthropic",
            Provider::OpenAi => "OpenAI",
            Provider::Ollama => "Ollama",
        }
    }

    pub fn env_var(self) -> &'static str {
        match self {
            Provider::Claude => "ANTHROPIC_API_KEY",
            Provider::OpenAi => "OPENAI_API_KEY",
            Provider::Ollama => "OLLAMA_HOST",
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
}

impl std::ops::AddAssign for Usage {
    fn add_assign(&mut self, o: Usage) {
        self.input_tokens += o.input_tokens;
        self.output_tokens += o.output_tokens;
        self.cache_read_tokens += o.cache_read_tokens;
    }
}

/// One model turn: the JSON it produced, and the assistant message to append
/// to the conversation exactly as returned.
pub struct Turn {
    pub json: String,
    pub assistant: Value,
    pub usage: Usage,
}

pub struct Client {
    pub provider: Provider,
    pub model: String,
    key: String,
    /// Ollama's address (unused for hosted providers).
    base_url: String,
    http: reqwest::blocking::Client,
}

/// Context window requested from Ollama. Its default is small enough to
/// silently cut off the system prompt and worked example.
const OLLAMA_CONTEXT: u32 = 32_768;

/// Claude models that accept the server-side refusal fallback.
const FALLBACK_MODELS: &[&str] = &[
    "claude-fable-5-1",
    "claude-opus-5-5",
    "claude-opus-5",
    "claude-sonnet-5-5",
];

impl Client {
    pub fn new(provider: Provider, model: String, key: String, base_url: String) -> Result<Client> {
        let http = reqwest::blocking::Client::builder()
            // Drafting a whole question takes minutes, more on a local model.
            .timeout(Duration::from_secs(if provider == Provider::Ollama {
                45 * 60
            } else {
                15 * 60
            }))
            .build()?;
        Ok(Client {
            provider,
            model,
            key,
            base_url: base_url.trim_end_matches('/').to_string(),
            http,
        })
    }

    pub fn user(text: &str) -> Value {
        json!({ "role": "user", "content": text })
    }

    /// Asks for a JSON document matching `schema`. `messages` alternate user
    /// and assistant turns and are only ever appended to.
    pub fn complete(&self, system: &str, messages: &[Value], schema: &Value) -> Result<Turn> {
        match self.provider {
            Provider::Claude => self.claude(system, messages, schema),
            Provider::OpenAi => self.openai(system, messages, schema),
            Provider::Ollama => self.ollama(system, messages, schema),
        }
    }

    /// Ollama's native chat API: `format` takes the JSON schema directly.
    fn ollama(&self, system: &str, messages: &[Value], schema: &Value) -> Result<Turn> {
        let mut all = vec![json!({ "role": "system", "content": system })];
        all.extend(messages.iter().cloned());
        let body = json!({
            "model": self.model,
            "messages": all,
            "stream": false,
            "format": schema,
            "options": { "num_ctx": OLLAMA_CONTEXT, "temperature": 0.2 },
        });
        let resp = self.send(|| self.http.post(format!("{}/api/chat", self.base_url)).json(&body))?;
        if resp["done_reason"] == "length" {
            bail!("the draft ran past the model's output limit; try a smaller question");
        }
        let text = resp["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .trim()
            .to_string();
        if text.is_empty() {
            bail!("the model returned nothing; try again, or a different model in /config");
        }
        Ok(Turn {
            assistant: json!({ "role": "assistant", "content": text }),
            json: text,
            usage: Usage {
                input_tokens: resp["prompt_eval_count"].as_u64().unwrap_or(0),
                output_tokens: resp["eval_count"].as_u64().unwrap_or(0),
                cache_read_tokens: 0,
            },
        })
    }

    fn claude(&self, system: &str, messages: &[Value], schema: &Value) -> Result<Turn> {
        let mut body = json!({
            "model": self.model,
            "max_tokens": 32000,
            // Stable prefix: cached across the drafting rounds.
            "system": [{ "type": "text", "text": system, "cache_control": { "type": "ephemeral" } }],
            "messages": messages,
            "output_config": {
                "effort": "high",
                "format": { "type": "json_schema", "schema": schema }
            },
        });
        let fallback = FALLBACK_MODELS.contains(&self.model.as_str());
        if fallback {
            body["fallbacks"] = json!("default");
        }
        let resp = self.send(|| {
            let mut req = self
                .http
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &self.key)
                .header("anthropic-version", "2023-06-01")
                .json(&body);
            if fallback {
                req = req.header("anthropic-beta", "server-side-fallback-2026-07-01");
            }
            req
        })?;

        match resp["stop_reason"].as_str() {
            Some("refusal") => bail!(
                "the model declined to draft this question{}",
                resp["stop_details"]["explanation"]
                    .as_str()
                    .map(|e| format!(": {e}"))
                    .unwrap_or_default()
            ),
            Some("max_tokens") => {
                bail!("the draft ran past the output limit; try a smaller question")
            }
            _ => {}
        }
        let content = resp["content"].clone();
        let text: String = content
            .as_array()
            .context("response has no content")?
            .iter()
            .filter(|b| b["type"] == "text")
            .filter_map(|b| b["text"].as_str())
            .collect();
        let u = &resp["usage"];
        Ok(Turn {
            json: text,
            assistant: json!({ "role": "assistant", "content": content }),
            usage: Usage {
                input_tokens: u["input_tokens"].as_u64().unwrap_or(0)
                    + u["cache_creation_input_tokens"].as_u64().unwrap_or(0),
                output_tokens: u["output_tokens"].as_u64().unwrap_or(0),
                cache_read_tokens: u["cache_read_input_tokens"].as_u64().unwrap_or(0),
            },
        })
    }

    fn openai(&self, system: &str, messages: &[Value], schema: &Value) -> Result<Turn> {
        let mut all = vec![json!({ "role": "system", "content": system })];
        all.extend(messages.iter().cloned());
        let body = json!({
            "model": self.model,
            "messages": all,
            "response_format": {
                "type": "json_schema",
                "json_schema": { "name": "dojo_question", "strict": true, "schema": schema }
            },
        });
        let resp = self.send(|| {
            self.http
                .post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(&self.key)
                .json(&body)
        })?;
        let msg = &resp["choices"][0]["message"];
        if let Some(refusal) = msg["refusal"].as_str() {
            bail!("the model declined to draft this question: {refusal}");
        }
        if resp["choices"][0]["finish_reason"] == "length" {
            bail!("the draft ran past the output limit; try a smaller question");
        }
        let text = msg["content"]
            .as_str()
            .context("response has no content")?
            .to_string();
        let u = &resp["usage"];
        Ok(Turn {
            assistant: json!({ "role": "assistant", "content": text }),
            json: text,
            usage: Usage {
                input_tokens: u["prompt_tokens"].as_u64().unwrap_or(0),
                output_tokens: u["completion_tokens"].as_u64().unwrap_or(0),
                cache_read_tokens: u["prompt_tokens_details"]["cached_tokens"]
                    .as_u64()
                    .unwrap_or(0),
            },
        })
    }

    /// Sends with retries on rate limits, overload and server errors.
    fn send(&self, request: impl Fn() -> reqwest::blocking::RequestBuilder) -> Result<Value> {
        let mut delay = Duration::from_secs(2);
        for attempt in 0..4 {
            let result = request().send();
            let resp = match result {
                Ok(r) => r,
                Err(e) if attempt < 3 && (e.is_connect() || e.is_timeout()) => {
                    std::thread::sleep(delay);
                    delay *= 2;
                    continue;
                }
                Err(e) => {
                    return Err(anyhow!(
                        "could not reach {}: {e}{}",
                        self.provider.label(),
                        if self.provider == Provider::Ollama {
                            "  ·  is `ollama serve` running?"
                        } else {
                            ""
                        }
                    ));
                }
            };
            let status = resp.status();
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);
            let body: Value = resp.json().unwrap_or(Value::Null);
            if status.is_success() {
                return Ok(body);
            }
            // Hosted APIs nest the message; Ollama returns {"error": "..."}.
            let message = body["error"]["message"]
                .as_str()
                .or(body["error"].as_str())
                .unwrap_or("no details")
                .to_string();
            let retryable =
                status.as_u16() == 429 || status.as_u16() == 529 || status.is_server_error();
            if retryable && attempt < 3 {
                std::thread::sleep(retry_after.unwrap_or(delay).min(Duration::from_secs(60)));
                delay *= 2;
                continue;
            }
            bail!(
                "{} returned {}: {message}{}",
                self.provider.label(),
                status.as_u16(),
                match status.as_u16() {
                    401 => "  ·  check your API key (/contribute key)",
                    404 => "  ·  check the model name in /config",
                    _ => "",
                }
            );
        }
        bail!("{} is busy; try again in a minute", self.provider.label())
    }
}

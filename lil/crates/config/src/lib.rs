use anyhow::{Context, Result};
use liquid::{object, Template};
use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_PROMPT: &str = "You are a helpful shell assistant. Here is the recent history of the shell session:\n---\n{{context}}\n---\nGiven the history, what is the user asking for in their latest prompt: '{{user_input}}'?";

use std::fmt;

#[derive(Deserialize, Debug)]
struct RawConfig {
    prompt: String,
}

pub struct Config {
    pub prompt: Template,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
         .field("prompt", &"<Liquid Template>")
         .finish()
    }
}

impl Config {
    pub fn render_prompt(&self, context: &str, user_input: &str) -> Result<String> {
        let globals = object!({
            "context": context,
            "user_input": user_input,
        });
        let output = self.prompt.render(&globals)?;
        Ok(output)
    }
}

fn default_config() -> Config {
    let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let prompt = parser.parse(DEFAULT_PROMPT).unwrap();
    Config { prompt }
}

pub async fn load() -> Result<Config> {
    let config_path_str = "~/.config/lil/lil.toml";
    let expanded_path = shellexpand::tilde(config_path_str);
    let config_path = PathBuf::from(expanded_path.as_ref());

    if !config_path.exists() {
        return Ok(default_config());
    }

    let content = tokio::fs::read_to_string(config_path).await?;
    let raw_config: RawConfig = toml::from_str(&content)
        .with_context(|| "Failed to parse lil.toml")?;

    let parser = liquid::ParserBuilder::with_stdlib().build()?;
    let prompt = parser.parse(&raw_config.prompt)
        .with_context(|| "Failed to parse liquid template from lil.toml")?;

    Ok(Config { prompt })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_default_config() {
        // We can't easily test the file-based loading in a unit test
        // without manipulating the user's actual home directory.
        // We will trust the happy path and focus on default creation.
        let config = default_config();
        let rendered = config.render_prompt("test_context", "test_input").unwrap();
        assert!(rendered.contains("test_context"));
        assert!(rendered.contains("test_input"));
    }
}

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub num_players: i32,
    pub model_path: String,
    pub llama_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMClientResponse {
    pub response: String,
    pub success: bool,
}

pub struct LLMClient {
    llama_path: String,
    model_path: String,
}

impl LLMClient {
    pub fn new(llama_path: String, model_path: String) -> Self {
        Self {
            llama_path,
            model_path,
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(config.llama_path.clone(), config.model_path.clone())
    }

    pub fn generate(&mut self, prompt: &str, context: &[String]) -> Result<String> {
        // Combine context and prompt
        let full_prompt = if context.is_empty() {
            prompt.to_string()
        } else {
            format!(
                "{}\n\nBased on this context:\n{}",
                prompt,
                context.join("\n")
            )
        };

        // Run the model
        let response = self.run(&full_prompt)?;

        if (!response.success) {
            anyhow::bail!("Failed to generate response");
        }

        Ok(response.response.trim().to_string())
    }

    pub fn run(&self, prompt: &str) -> Result<LLMClientResponse> {
        let mut prompt_file = NamedTempFile::new()?;
        std::io::Write::write_all(&mut prompt_file, prompt.as_bytes())?;
        println!("Using model path: {}", self.model_path);
        println!("Using llama binary: {}", self.llama_path);

        // Build the command arguments with options to prevent interactive mode
        let args = [
            "-m",
            &self.model_path,
            "--prompt",
            prompt,
            "--json-schema",
            "{\"type\": \"object\", \"properties\": {\"vote\": { \"type\": \"number\" }}}",
            "-st", // Single turn
            "--no-display-prompt",
            "--n-gpu-layers",
            "32",
        ];

        // Run the llama-cpp command
        let output = Command::new(&self.llama_path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute llama-cpp")?;

        let success = output.status.success();
        let response = String::from_utf8_lossy(&output.stdout).to_string();

        if (!success) {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error running llama-cpp: {}", error);
        }

        Ok(LLMClientResponse { response, success })
    }

    pub fn check_files_exist(&self) -> bool {
        let llama_exists = Path::new(&self.llama_path).exists();
        let model_exists = Path::new(&self.model_path).exists();

        if (!llama_exists) {
            eprintln!("Llama binary not found at: {}", self.llama_path);
        }

        if (!model_exists) {
            eprintln!("Model file not found at: {}", self.model_path);
        }

        llama_exists && model_exists
    }
}

pub struct MockLLMClient {
    responses: Vec<String>,
    current_response: usize,
}

impl MockLLMClient {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            current_response: 0,
        }
    }

    pub fn add_response(&mut self, response: String) {
        self.responses.push(response);
    }

    pub fn generate(&mut self, prompt: &str, _context: &[String]) -> Result<String> {
        if self.current_response >= self.responses.len() {
            anyhow::bail!("No more mock responses available");
        }

        let response = self.responses[self.current_response].clone();
        self.current_response += 1;
        Ok(response)
    }
}

pub trait LLM {
    fn generate(&mut self, prompt: &str, context: &[String]) -> Result<String>;
}

impl LLM for LLMClient {
    fn generate(&mut self, prompt: &str, context: &[String]) -> Result<String> {
        self.generate(prompt, context)
    }
}

impl LLM for MockLLMClient {
    fn generate(&mut self, prompt: &str, context: &[String]) -> Result<String> {
        self.generate(prompt, context)
    }
}

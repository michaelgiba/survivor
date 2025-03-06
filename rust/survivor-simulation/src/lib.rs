use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub struct LlamaSimulation {
    llama_path: String,
    model_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationResult {
    pub response: String,
    pub success: bool,
}

impl LlamaSimulation {
    /// Create a new LlamaSimulation instance
    pub fn new(llama_path: String, model_path: String) -> Self {
        Self {
            llama_path,
            model_path,
        }
    }

    /// Create a new LlamaSimulation instance with default paths
    pub fn default() -> Self {
        let base_dir = std::env::current_dir().expect("Failed to get current directory");
        let root_dir = base_dir.parent().expect("Failed to get parent directory");

        let llama_path = root_dir
            .join("ext")
            .join("llama.cpp")
            .join("build")
            .join("bin")
            .join("llama-cpp")
            .to_string_lossy()
            .to_string();

        let model_path = root_dir
            .join("models")
            .join("microsoft_Phi-4-mini-instruct-IQ4_XS.gguf")
            .to_string_lossy()
            .to_string();

        Self::new(llama_path, model_path)
    }

    /// Run a simulation with the given prompt
    pub fn run(&self, prompt: &str) -> Result<SimulationResult> {
        // Create a temporary file for the prompt
        let mut prompt_file = NamedTempFile::new()?;
        std::io::Write::write_all(&mut prompt_file, prompt.as_bytes())?;

        // Run the llama-cpp command
        let output = Command::new(&self.llama_path)
            .args([
                "-m",
                &self.model_path,
                "--file",
                prompt_file.path().to_str().unwrap(),
                "--temp",
                "0.7",
                "--top-p",
                "0.9",
                "--max-tokens",
                "300",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute llama-cpp")?;

        let success = output.status.success();
        let response = String::from_utf8_lossy(&output.stdout).to_string();

        if !success {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error running llama-cpp: {}", error);
        }

        Ok(SimulationResult { response, success })
    }

    /// Check if the model and binary exist
    pub fn check_files_exist(&self) -> bool {
        Path::new(&self.llama_path).exists() && Path::new(&self.model_path).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_creation() {
        let simulation = LlamaSimulation::new("test_path".to_string(), "test_model".to_string());
        assert_eq!(simulation.llama_path, "test_path");
        assert_eq!(simulation.model_path, "test_model");
    }
}

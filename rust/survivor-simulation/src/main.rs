use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    num_players: i32,
    model_path: String,
    llama_path: String, // Changed to required instead of optional
}

#[derive(Debug, Serialize, Deserialize)]
struct SimulationResult {
    response: String,
    success: bool,
}

struct LlamaSimulation {
    llama_path: String,
    model_path: String,
}

impl LlamaSimulation {
    /// Create a new LlamaSimulation instance
    fn new(llama_path: String, model_path: String) -> Self {
        Self {
            llama_path,
            model_path,
        }
    }

    /// Create a new LlamaSimulation instance from config
    fn from_config(config: &Config) -> Self {
        Self::new(config.llama_path.clone(), config.model_path.clone())
    }

    /// Run a simulation with the given prompt
    fn run(&self, prompt: &str) -> Result<SimulationResult> {
        // Create a temporary file for the prompt
        let mut prompt_file = NamedTempFile::new()?;
        std::io::Write::write_all(&mut prompt_file, prompt.as_bytes())?;

        println!("Using model path: {}", self.model_path);
        println!("Using llama binary: {}", self.llama_path);

        // Build the command arguments with options to prevent interactive mode
        let args = [
            "-m",
            &self.model_path,
            "--prompt",
            "What is 4 + 4",
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

        if !success {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error running llama-cpp: {}", error);
        }

        Ok(SimulationResult { response, success })
    }

    /// Check if the model and binary exist
    fn check_files_exist(&self) -> bool {
        let llama_exists = Path::new(&self.llama_path).exists();
        let model_exists = Path::new(&self.model_path).exists();

        if !llama_exists {
            eprintln!("Llama binary not found at: {}", self.llama_path);
        }

        if !model_exists {
            eprintln!("Model file not found at: {}", self.model_path);
        }

        llama_exists && model_exists
    }
}

fn main() -> Result<()> {
    // Get the command line argument for the config file path
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config.json>", args[0]);
        std::process::exit(1);
    }

    let config_path = PathBuf::from(&args[1]);

    // Read and parse the JSON file
    let file = File::open(&config_path)
        .with_context(|| format!("Failed to open config file: {:?}", config_path))?;
    let reader = BufReader::new(file);
    let config: Config =
        serde_json::from_reader(reader).context("Failed to parse JSON config file")?;

    println!("Number of players: {}", config.num_players);

    // Initialize the simulation
    let simulation = LlamaSimulation::from_config(&config);

    // Check if the required files exist
    if !simulation.check_files_exist() {
        eprintln!("Error: llama-cpp binary or model file not found");
        std::process::exit(1);
    }

    // Create the prompt
    let prompt = format!("What is {} + 1?", config.num_players);

    // Run the simulation
    match simulation.run(&prompt) {
        Ok(result) => {
            if result.success {
                println!("{}", result.response);
            } else {
                eprintln!("Model execution failed");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error running simulation: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

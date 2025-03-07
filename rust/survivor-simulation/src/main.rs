use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod llm;
mod simulation;

use llm::{Config, LLMClient};

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
    let simulation = LLMClient::from_config(&config);

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

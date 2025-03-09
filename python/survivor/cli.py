import argparse
import json
import os
from survivor.llm_server import LlamaServer
from survivor.events import EventBuffer
from survivor._simulation import SurvivorSim


def _start_sim(args):
    """
    Run a survivor simulation using the specified model and configuration.
    """
    # Validate paths
    if not os.path.exists(args.model):
        raise ValueError(f"Model file not found: {args.model}")
    if not os.path.exists(args.config):
        raise ValueError(f"Config file not found: {args.config}")

    # Load configuration JSON
    with open(args.config, "r") as f:
        config = json.load(f)

    # Create output directory if it doesn't exist
    os.makedirs(os.path.dirname(args.output), exist_ok=True)

    event_buffer = EventBuffer([])
    # Initialize and run simulation using context manager
    print("Starting LLama.cpp server...")
    with LlamaServer(args.model) as server:
        print("Starting simulation...")
        SurvivorSim(config, event_buffer).execute()
        print(f"Simulation complete. Results written to: {args.output}")

    print(event_buffer.full_text())


def main():
    parser = argparse.ArgumentParser(description="Run survivor simulation with LLM")
    parser.add_argument("--model", required=True, help="Path to the GGML model file")
    parser.add_argument(
        "--config", required=True, help="Path to the configuration JSON file"
    )
    parser.add_argument(
        "--output", required=True, help="Path to write simulation results"
    )

    args = parser.parse_args()
    _start_sim(args)


if __name__ == "__main__":
    main()

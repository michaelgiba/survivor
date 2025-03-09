import argparse
import json
import os
from _llm_server import LlamaServer
from _simulation.survivor import SurvivorSimulation


def simulate_survivor(args):
    """
    Run a survivor simulation using the specified model and configuration.
    """
    # Validate paths
    if not os.path.exists(args.model):
        raise ValueError(f"Model file not found: {args.model}")
    if not os.path.exists(args.config):
        raise ValueError(f"Config file not found: {args.config}")

    # Create output directory if it doesn't exist
    os.makedirs(os.path.dirname(args.output), exist_ok=True)

    # Initialize and start the LLM server
    print("Starting LLama.cpp server...")
    server = LlamaServer(args.model)
    server.start()

    try:
        # Initialize and run simulation
        print("Starting simulation...")
        simulation = SurvivorSimulation(args.config, server)
        results = simulation.run()

        # Save results
        simulation.save_results(args.output)
        print(f"Simulation complete. Results written to: {args.output}")

    finally:
        # Ensure server is stopped
        print("Stopping LLama.cpp server...")
        server.stop()


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
    simulate_survivor(args)


if __name__ == "__main__":
    main()

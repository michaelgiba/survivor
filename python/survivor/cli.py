import argparse
import json
import os
from survivor.events import EventBuffer
from survivor._simulation import SurvivorSim
from survivor.llm_util import (
    LLMBackendType,
    BACKEND_TYPE_TO_BACKEND,
    ACTIVE_GENERAL_TYPE,
)
from survivor.llm_util import _local_backend


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

    # Configure the local backend with the specified model
    _local_backend.DEFAULT_MODEL_PATH = args.model

    event_buffer = EventBuffer([])

    print("Starting simulation...")
    SurvivorSim(config, event_buffer).execute()
    print(f"Simulation complete. Results written to: {args.output}")

    output_data = event_buffer.to_json(indent=2)
    print(output_data)


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

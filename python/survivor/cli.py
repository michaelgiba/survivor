import argparse
import json
import os
from survivor._simulation import SurvivorSim
from survivor.llm_util import LLMBackendType, set_general_model_type
from survivor.llm_util import _local_backend
import plomp


def _start_sim(args):
    """
    Run a survivor simulation using the specified model and configuration.
    """
    # Validate paths
    if args.model and not os.path.exists(args.model):
        raise ValueError(f"Model file not found: {args.model}")
    if not os.path.exists(args.config):
        raise ValueError(f"Config file not found: {args.config}")

    # Load configuration JSON
    with open(args.config, "r") as f:
        config = json.load(f)

    print("Starting simulation...")

    def write_progress():
        plomp.write_html(plomp.buffer(), args.output_uri)

    SurvivorSim(config).execute(write_progress=write_progress)

    write_progress()


def main():
    parser = argparse.ArgumentParser(description="Run survivor simulation with LLM")

    parser.add_argument(
        "--backend",
        choices=[backend.name for backend in LLMBackendType],
        default=LLMBackendType.GROQ.name,
        help="Backend type for LLM processing",
    )

    parser.add_argument(
        "--model", help="Path to the GGML model file (required for local backend)"
    )

    parser.add_argument(
        "--config", required=True, help="Path to the configuration JSON file"
    )

    parser.add_argument(
        "--output-uri", required=True, help="The URI to render the playback with plomp"
    )

    args = parser.parse_args()

    # Check if model is required based on backend type

    set_general_model_type(LLMBackendType[args.backend])

    if args.backend == LLMBackendType.LOCAL:
        if args.model:
            _local_backend.DEFAULT_MODEL_PATH = args.model
        else:
            parser.error("--model is required when using the local backend")

    if args.backend != LLMBackendType.LOCAL and args.model:
        parser.error(
            "It doesn't make sense to specify --model if the local backend is not used."
        )

    _start_sim(args)


if __name__ == "__main__":
    main()

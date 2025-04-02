import subprocess
import requests
import time
import os
from typing import Optional, Dict, Any


class LlamaServer:
    def __init__(self, model_path: str, host: str = "127.0.0.1", port: int = 8080):
        self.model_path = model_path
        self.host = host
        self.port = port
        self.process: Optional[subprocess.Popen] = None
        self.base_url = f"http://{host}:{port}/completion"
        # Compute the project root (assumes this file is in a subfolder of the project root)
        self.project_root = os.path.abspath(
            os.path.join(os.path.dirname(__file__), "..", "..", "..")
        )
        # Build the full path to the llama-server executable
        self.executable = os.path.join(
            self.project_root, "ext", "llama.cpp", "build", "bin", "llama-server"
        )

    def start(self) -> None:
        """Start the llama.cpp server using llama-server executable"""
        if self.process:
            return
        cmd = [
            self.executable,
            "-m",
            self.model_path,
            "--host",
            self.host,
            "--port",
            str(self.port),
        ]
        # Print the full command
        print(f"Executing command: {' '.join(cmd)}")
        # Optionally, set the working directory to the directory containing the executable
        cwd = os.path.dirname(self.executable)
        # Change to not pipe the output
        self.process = subprocess.Popen(
            cmd,
            cwd=cwd,
            # Remove the pipe and let output go to terminal
            text=True,
        )

    def stop(self) -> None:
        """Stop the llama.cpp server"""
        if self.process:
            self.process.terminate()
            self.process.wait()
            self.process = None

    def query(self, prompt: str, **kwargs) -> Dict[str, Any]:
        """Send a query to the llama server"""
        data = {
            "prompt": prompt,
            "n_predict": kwargs.get("n_predict", 128),
            "temperature": kwargs.get("temperature", 0.8),
            "top_p": kwargs.get("top_p", 0.95),
            "stop": kwargs.get("stop", []),
        }
        if "json_schema" in kwargs:
            data["json_schema"] = kwargs["json_schema"]
        response = requests.post(self.base_url, json=data)
        response.raise_for_status()
        return response.json()

    def __enter__(self):
        """Context manager entry"""
        self.start()
        self.wait_until_healthy()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.stop()

    def wait_until_healthy(self, timeout=60, check_interval=1):
        """
        Wait until the server is healthy and responding to requests

        Args:
            timeout: Maximum time to wait in seconds
            check_interval: Time between health checks in seconds

        Raises:
            TimeoutError: If server doesn't become healthy within timeout period
        """
        print(
            f"Waiting for LLama server to become healthy at {self.host}:{self.port}..."
        )
        health_url = f"http://{self.host}:{self.port}/health"
        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                response = requests.get(health_url, timeout=2)
                if response.status_code == 200:
                    print(
                        f"LLama server is healthy after {time.time() - start_time:.1f} seconds"
                    )
                    return True
            except requests.RequestException:
                pass

            time.sleep(check_interval)

        raise TimeoutError(
            f"LLama server failed to become healthy within {timeout} seconds"
        )


# Default model path - adjust as necessary for your environment
DEFAULT_MODEL_PATH = os.path.join(
    os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..")),
    "models",
    "microsoft_Phi-4-mini-instruct-IQ4_XS.gguf",
)

# Singleton LlamaServer instance
_llama_server = None


def get_llama_server(model_path=DEFAULT_MODEL_PATH):
    """Get or create a singleton LlamaServer instance and ensure it's started"""
    global _llama_server
    if _llama_server is None:
        _llama_server = LlamaServer(model_path)
        _llama_server.start()
        _llama_server.wait_until_healthy()
    return _llama_server


def completion(
    prompt: str,
    system_prompt: str = "",
    temperature: float = 0.8,
    response_json_schema: dict | None = None,
) -> Dict[str, Any]:
    """
    Generate a completion using the local LLM backend.

    Args:
        prompt: The user prompt to send to the model
        system_prompt: Optional system prompt to control model behavior
        temperature: Controls randomness in the generation (0.0-1.0)
        force_json: Whether to force the response to be in JSON format

    Returns:
        A dictionary with the completion response formatted to match the API
        response structure used by other backends
    """
    server = get_llama_server()

    full_prompt = f"{system_prompt}\n\n{prompt}" if system_prompt else prompt

    kwargs = {
        "temperature": temperature,
    }

    if response_json_schema:
        kwargs["json_schema"] = response_json_schema

    # Query the server
    raw_response = server.query(full_prompt, **kwargs)

    # Format response to match the structure expected by the calling code
    formatted_response = {
        "choices": [
            {
                "message": {
                    "content": raw_response.get("content", ""),
                    "role": "assistant",
                },
                "finish_reason": raw_response.get("finish_reason", "stop"),
            }
        ]
    }

    return formatted_response

import subprocess
import requests
import time
import os
from contextlib import contextmanager
from typing import Optional, Dict, Any


class LlamaServer:
    def __init__(self, model_path: str, host: str = "127.0.0.1", port: int = 8080):
        self.model_path = model_path
        self.host = host
        self.port = port
        self.process: Optional[subprocess.Popen] = None
        self.base_url = f"http://{host}:{port}/completion"

    def start(self) -> None:
        """Start the llama.cpp server"""
        if self.process:
            return

        cmd = [
            "./build/bin/server",
            "-m",
            self.model_path,
            "--host",
            self.host,
            "--port",
            str(self.port),
        ]

        self.process = subprocess.Popen(
            cmd,
            cwd=os.path.join(os.path.dirname(__file__), "ext", "llama.cpp"),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        # Wait for server to start
        max_retries = 10
        for _ in range(max_retries):
            try:
                requests.get(f"http://{self.host}:{self.port}/health")
                return
            except requests.exceptions.ConnectionError:
                time.sleep(1)

        raise RuntimeError("Failed to start llama server")

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

        response = requests.post(self.base_url, json=data)
        response.raise_for_status()
        return response.json()

    def __enter__(self):
        """Context manager entry"""
        self.start()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.stop()

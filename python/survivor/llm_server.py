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
            os.path.join(os.path.dirname(__file__), "..", "..")
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
        # self.start()
        # time.sleep(3.0)

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.stop()

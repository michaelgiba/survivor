import time

import requests
import os

_GROQ_APIKEY = os.environ.get("GROQ_API_KEY")


def completion(
    prompt: str,
    system_prompt: str,
    temperature: float,
    *,
    response_json_schema: dict | None = None,
) -> str:
    time.sleep(1.0)
    result = requests.post(
        "https://api.groq.com/openai/v1/chat/completions",
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {_GROQ_APIKEY}",
        },
        json={
            "model": "llama3-8b-8192",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": prompt},
            ],
            "max_tokens": 4096,
            "temperature": temperature,
            **(
                {"response_format": {"type": "json_object"}}
                if response_json_schema is not None
                else {}
            ),
        },
    )
    return result.json()

import xml.etree.ElementTree as ET
from enum import Enum, auto

from survivor.llm_util import _groq_backend, _local_backend, _openai_backend


class LLMBackendType(Enum):
    GROQ = auto()
    LOCAL = auto()
    OPENAI = auto()


BACKEND_TYPE_TO_BACKEND = {
    LLMBackendType.GROQ: _groq_backend,
    LLMBackendType.LOCAL: _local_backend,
    LLMBackendType.OPENAI: _openai_backend,
}

# Tweak backend here
ACTIVE_GENERAL_TYPE = LLMBackendType.LOCAL
ACTIVE_GENERAL_BACKEND = BACKEND_TYPE_TO_BACKEND[ACTIVE_GENERAL_TYPE]


def prompt_general_info_extraction(prompt: str):
    json_response = ACTIVE_GENERAL_BACKEND.completion(
        prompt,
        system_prompt="You are a general purpose LLM helping with information extraction",
        temperature=0.9,
        force_json=True,
    )
    # print(json_response)
    content = json_response["choices"][0]["message"]["content"]
    return content.removesuffix("<|eot_id|>")


def prompt(prompt: str, system_prompt: str, temperature: float):
    json_response = ACTIVE_GENERAL_BACKEND.completion(
        prompt, system_prompt=system_prompt, temperature=temperature
    )
    # print(json_response)
    content = json_response["choices"][0]["message"]["content"]
    return content.removesuffix("<|eot_id|>")

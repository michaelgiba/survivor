#!/usr/bin/env bash
set -euxo pipefail

MODEL_NAME="bartowski/microsoft_Phi-4-mini-instruct-GGUF/resolve/main/microsoft_Phi-4-mini-instruct-IQ4_XS.gguf"
MODEL_DIR="models"
MODEL_FILE="${MODEL_DIR}/$(basename "$MODEL_NAME")"
REPO_URL="https://github.com/ggerganov/llama.cpp.git"

if [ ! -f "$MODEL_FILE" ]; then
    mkdir -p "$MODEL_DIR"
    cd "$MODEL_DIR"
    wget "https://huggingface.co/$MODEL_NAME"
    cd ..
fi

pushd .
cd ext

if [ ! -d "llama.cpp" ]; then
    git clone "$REPO_URL"
fi

cd "llama.cpp"
cmake -B build -DLLAMA_CURL=OFF -DGGML_CUDA=ON
cmake --build build --config Release

popd
#!/bin/bash

cd $(dirname $0)
./build.sh

cd python/

python -m survivor.cli \
    --config config.json \
    --backend GROQ \
    --output ./plomp.html

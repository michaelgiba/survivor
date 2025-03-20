#!/bin/bash

cd $(dirname $0)
./build.sh

cd typescript/
npx vite
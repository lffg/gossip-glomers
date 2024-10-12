#!/usr/bin/env bash

prog=$1

cargo build -p "$prog" --release
echo "running '$prog'"

maelstrom test \
    -w "$prog" \
    --bin "./target/release/$prog" \
    --node-count 1 \
    --time-limit 10

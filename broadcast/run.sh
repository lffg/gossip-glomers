#!/usr/bin/env bash
set -e

maelstrom test -w broadcast \
    --bin "$GG_PROG" \
    --node-count 5 --time-limit 20 --rate 10

#!/usr/bin/env bash
set -e

maelstrom test -w unique-ids \
    --bin "$GG_PROG" \
    --time-limit 30 --rate 1000 --node-count 3 \
    --availability total --nemesis partition

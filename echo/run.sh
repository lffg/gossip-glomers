#!/usr/bin/env bash

maelstrom test -w echo \
    --bin "$GG_PROG" \
    --node-count 1 --time-limit 10

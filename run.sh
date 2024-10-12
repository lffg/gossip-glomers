#!/usr/bin/env bash

prog=$1

cargo build -p "$prog" --release

echo "running '$prog'"

export GG_PROG="./target/release/$prog"
./"$prog/run.sh"

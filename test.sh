#!/usr/bin/env bash

names=$(
    cargo tree -f '{p}' --depth 0 |
    awk 'NF {print $1}' |
    grep -v proto
)

final=0
for name in $names; do
    ./run.sh "$name"
    status="$?"
    if (( status == 1 )); then
        echo "<!!!> $name FAILED!"
        final="$status"
    fi
done

if (( final == 1)); then
    echo; echo; echo
    echo "some tests failed!"
fi
exit "$final"

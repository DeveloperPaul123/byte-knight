#!/bin/bash

# read 2 args, the fen and the depth
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <fen> <depth>"
    exit 1
fi

fen=$1
depth=$2

# run our perft

./target/debug/perft --fen "$fen" -d "$depth" -s

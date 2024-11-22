#!/bin/bash

cutechess-cli -variant standard -concurrency 12 -games 768 \
    -engine dir=./target/debug cmd="./byte-knight" proto=uci tc=2.27+0.02 timemargin=250 option.Threads=1 option.Hash=16 name=ByteKnight-dev \
    -engine dir=./target/debug cmd="./byte-knight" proto=uci tc=2.27+0.02 timemargin=250 option.Threads=1 option.Hash=16 name=ByteKnight-base \
    -openings file="./data/Pohl.epd" format=epd order=random start=1921 \
    -srand 31 \
    -pgnout 31.48.1732290528.0.pgn \
    # -debug

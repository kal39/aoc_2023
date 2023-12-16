#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "USAGE: $0 day"
    exit 1
fi

SRC_DIR="src/bin/"
INPUT_DIR="input/"

mkdir -p $SRC_DIR
mkdir -p $INPUT_DIR

touch "$INPUT_DIR/day_$1.txt"
touch "$SRC_DIR/day_$1_part_1.rs"
touch "$SRC_DIR/day_$1_part_2.rs"

echo "fn main() {
}" > "$SRC_DIR/day_$1_part_1.rs"

echo "fn main() {
}" > "$SRC_DIR/day_$1_part_2.rs"
#!/bin/sh

mkdir "day$1"

curl "https://adventofcode.com/2021/day/$1/input" \
  -H 'authority: adventofcode.com' \
  -H "cookie: session=$(cat ./.cookie.txt)" \
  --compressed \
  -o "day$1/input"

cd "day$1" 

cargo init --bin --vcs=none

echo "easy_io = "0.3.0"" >> Cargo.toml

echo "\nProblem description here!"
echo "https://adventofcode.com/2021/day/$1"

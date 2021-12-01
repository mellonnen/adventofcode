#!/bin/zsh

day=$(date | awk '{ print $3 }')
mkdir "day$day"

curl "https://adventofcode.com/2021/day/$day/input" \
  -H 'authority: adventofcode.com' \
  -H "cookie: session=$(cat ./.cookie.txt)" \
  --compressed \
  -o "day$day/input"

cd "day$day" 

cargo init --bin --vcs=none

echo 'easy_io = "0.3.0"' >> Cargo.toml

wsl-open "https://adventofcode.com/2021/day/$day"

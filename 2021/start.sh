#!/bin/sh

# Get day from date.
day=$(date | awk '{ print $3 }')

# check if we have a single digit day number.
if [ $(($day+0)) -lt 10 ]
then
  dir="day0$day"
else
  dir="day$day"
fi

# create directory.
mkdir "$dir"

# Get input.
curl "https://adventofcode.com/2021/day/$day/input" \
  -H 'authority: adventofcode.com' \
  -H "cookie: session=$(cat ./.cookie.txt)" \
  --compressed \
  -o "$dir/input"

cd "$dir" 

# init rust project.
cargo init --bin --vcs=none

echo "$(cat ../boilerplate)" > src/main.rs
# open problem description in browser.
wsl-open "https://adventofcode.com/2021/day/$day"

#!/bin/bash

mylang="./target/debug/my-lang-rust"

runtest() {
  input="$1"
  expected="$2"
  output=$(RUST_BACKTRACE=1 $mylang $input)

  # error
  if [ "$expected" != "$output" ]; then
    echo "$input: $expected expected, but got $output"
    exit 1
  fi

  # success
  echo "$input => $output"
}

# compile -> ./target/debug/my-lang-rust
# cargo build --release
cargo build


# test code

runtest "1" "1"
runtest "5" 5

echo OK
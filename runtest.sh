#!/bin/bash

mylang="./target/debug/my-lang-rust"

runtest() {
  input="$1"
  expected="$2"
  output=$(RUST_BACKTRACE=1 "$mylang" "$input") # ここのミスで3時間くらい潰した．くそっっっっっ．
  # $で囲った中でも，変数使う時はダブルクオートいるんだよ

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
runtest " 5" 5
runtest "33" 33
runtest " 33" 33
runtest "    33    " 33


runtest '+ 1 2' '3'
runtest '+ 10 20' '30'
runtest '- 10 1' '9'
runtest '* 2 3' '6'
runtest '/ 10 5' '2'

echo OK
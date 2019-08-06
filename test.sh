#!/bin/bash

ninecc="./target/debug/nine-cc"

try() {
    expected="$1"
    input="$2"

    ${ninecc} "$input" > test.s
    gcc -static -o test test.s
    ./test
    actual="$?"

    if [ "$actual" != "$expected" ]; then
        echo "$input expected, but got $actual"
        exit 1
    fi
}

cargo build

try 0 0
try 42 42
try 41 " 12 + 34 - 5 "

echo OK

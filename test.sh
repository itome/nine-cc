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
try 90 '(12 + 3) * 6'
try 8 '4 + 8 / 2'
try 47 '5+6*7'
try 15 '5*(9-6)'
try 4 '(3+5)/2'
try 5 '+5'
try 5 '12+(-7)'
try 5 '20+(-3*5)'

# try 1 '5==5'
# try 1 '5<=5'
# try 1 '4 < 5'
# try 0 '7 < 5'
# try 1 '7 > 5'
# try 0 '4 > 5'
# try 0 '12+(-7) != 20+(-3*5)'

echo OK

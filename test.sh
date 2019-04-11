#!/bin/bash

function t() {
    target/debug/c8rs $2 > tmp.s
    gcc -o tmp tmp.s
    ./tmp

    RESULT=$?

    echo "$2 => $RESULT"
}

cargo build

t 1 1
t 21 "5+20-4"

rm -rf tmp tmp.s
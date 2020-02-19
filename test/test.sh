#!/bin/bash
try () {
    expected="$1"
    input="$2"

    cargo run -- "$input" > tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"
    if [ "$actual" = "$expected" ] ; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
    fi
    rm ./tmp.s ./tmp
}

try 0 0
try 42 42
try 21 "5+20-4"
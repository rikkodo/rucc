#!/bin/bash
try () {
    expected="$1"
    input="$2"

    actual=$(sh ./run.sh "$2")
    if [ "$actual" = "$expected" ] ; then
        echo  "\e[32m$input => $actual\e[0m"
    else
        echo "\e[31m$input => $expected expected, but got $actual\e[0m"
    fi
}

cargo build

try 0 0
try 42 42
try 21 "5+20-4"
try 41 " 12 +   34 - 5   "
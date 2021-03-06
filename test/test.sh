#!/bin/bash

DIR=$(dirname $0)

assert () {
    local expected="$1"
    local input="$2"

    local actual=$(sh ${DIR}/run.sh "$2")
    if [ "$actual" = "$expected" ] ; then
        echo  "\e[32m$input => $actual\e[0m"
    else
        echo "\e[31m$input => $expected expected, but got $actual\e[0m"
    fi
}

cargo build

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 +   34 - 5   "
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 10 '-10+20'
assert 10 '- -10'
assert 10 '- - +10'

assert 0 '0==1'
assert 1 '42==42'
assert 1 '0!=1'
assert 0 '42!=42'

assert 1 '0<1'
assert 0 '1<1'
assert 0 '2<1'
assert 1 '0<=1'
assert 1 '1<=1'
assert 0 '2<=1'

assert 1 '1>0'
assert 0 '1>1'
assert 0 '1>2'
assert 1 '1>=0'
assert 1 '1>=1'
assert 0 '1>=2'

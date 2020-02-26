#!/bin/bash

DIR=$(dirname $0)
DEBUGPATH="${DIR}/../target/debug/"
MODULE="rucc"

eval "${DEBUGPATH}${MODULE} \"${1}\" > tmp.s" &&\
gcc -o tmp tmp.s &&
./tmp; echo $?

rm -f tmp.s tmp
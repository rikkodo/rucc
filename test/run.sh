#!/bin/bash

DIR=$(dirname $0 [-p: print])
DEBUGPATH="${DIR}/../target/debug/"
MODULE="rucc"
PRINT=""

while :
do
    case $1 in
        -p) PRINT="1"; shift;;
        *)  break;;
    esac
done

eval "${DEBUGPATH}${MODULE} \"${1}\" > tmp.s" &&\
gcc -o tmp tmp.s &&\
./tmp; echo $?

if [ "${PRINT}" != "" ] ;
then
    cat tmp.s 1>&2
fi

rm -f tmp.s tmp

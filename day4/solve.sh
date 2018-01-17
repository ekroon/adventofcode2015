#!/usr/bin/env bash
if [ -z "$1" ] 
then
    echo "./solve.sh <input>"
else
    echo -n "part1: " && echo $1 | perl -MDigest::MD5=md5_hex -nE 'chomp; $input=$_; for ($i=0;;$i++) {if ("00000" eq substr md5_hex("${input}${i}"),0,5) {say $i; exit;}}'
    echo -n "part2: " && echo $1 | perl -MDigest::MD5=md5_hex -nE 'chomp; $input=$_; for ($i=0;;$i++) {if ("000000" eq substr md5_hex("${input}${i}"),0,6) {say $i; exit;}}'
fi

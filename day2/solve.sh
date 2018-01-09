#!/bin/env bash
if [ -z "$1" ] 
then
echo "./solve.sh <input_file_name>"
else 
echo -n "part1: " && perl -nE 'for(split//) {$i++;$x+=/\(/?1:-1;}; END {say $x}' < $1
echo -n "part2: " && perl -nE 'for(split//) {$i++;$x+=/\(/?1:-1; exit if $x < 0}; END {say $i}' < $1
fi

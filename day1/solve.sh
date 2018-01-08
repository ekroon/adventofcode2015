#!/bin/env bash
echo -n "part1: " && perl -nE 'for(split//) {$i++;$x+=/\(/?1:-1;}; END {say $x}' < input.txt
echo -n "part2: " && perl -nE 'for(split//) {$i++;$x+=/\(/?1:-1; exit if $x < 0}; END {say $i}' < input.txt
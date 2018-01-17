#!/usr/bin/env bash
if [ -z "$1" ] 
then
    echo "./solve.sh <input_file_name>"
else
    echo -n "part1: " && cat $1 | grep -E '[aoeiu]{1}.*[aoeiu]{1}.*[aoeiu]{1}' | grep -E '(.)\1{1}' | grep -v -E 'ab|cd|pq|xy' | wc -l | awk '{$1=$1};1'
    echo -n "part2: " && cat $1 | grep -E '(.).\1{1}' | grep -E '(..).*\1' | wc -l | awk '{$1=$1};1'
fi

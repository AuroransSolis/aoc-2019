#!/bin/bash

echo "Creating file with $(($1 * 1000)) lines."
> large_$1k.txt

for i in $(seq 1 "$1")
do
    shuf -i 10000-100000 -n 1000 >> large_$1k.txt
done

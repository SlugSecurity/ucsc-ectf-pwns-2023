#!/usr/bin/env bash
cat $2 > $1
sleep 1
cat unlock.bin > $1
# cat features.bin > $1
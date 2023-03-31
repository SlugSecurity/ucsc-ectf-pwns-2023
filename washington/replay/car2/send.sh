#!/usr/bin/env bash
for x in $(cat $2); do
    xxd -p -r <(echo $x) > $1
    sleep 0.0008
done
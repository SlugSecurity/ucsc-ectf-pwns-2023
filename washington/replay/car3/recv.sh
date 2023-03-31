#!/usr/bin/env bash
sleep 0.0008
echo -n A > $1
for x in $(seq 1 58); do
    sleep 0.0008
    echo -n A > $1
done
#!/usr/bin/env bash

xxd -p -r umass0.txt > $1
sleep 0.45
xxd -p -r umass1.txt > $1
sleep 0.01
xxd -p -r umass2.txt > $1
sleep 0.01
xxd -p -r umass3.txt > $1
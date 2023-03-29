#!/usr/bin/env bash
cat car5.bin > $1
sleep 1
cat unlock.bin > $1
sleep 1
cat car5_2.bin > $1
sleep 1
cat unlock.bin > $1
sleep 1
cat features_message.bin > $1
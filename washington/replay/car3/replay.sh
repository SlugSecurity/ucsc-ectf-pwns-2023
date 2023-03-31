#!/usr/bin/env bash

./send.sh $1 uw0.txt
sleep 0.001
./recv.sh $1
./send.sh $1 uw1.txt
sleep 0.001
./recv.sh $1
#!/bin/bash

FILE="$1"

BYTES_PER_ACK=8
ACK_MESSAGE="A"

while read -r -n $BYTES_PER_ACK bytes; do
    echo -n "$bytes"
    if [[ ${#bytes} -eq $BYTES_PER_ACK ]]; then
        echo -n "$ACK_MESSAGE"
        sleep 0.01
    fi
done < "$FILE"

#!/bin/bash

SERIAL_PORT="$1"
START_PIN=$((0x${2:-000000}))
STOP_PIN=$((0x${3:-ffffff}))

PIN=$START_PIN
while [[ $PIN -lt $STOP_PIN ]]; do
    printf 'pair%06x\n' "$PIN" > "$SERIAL_PORT"
    if [[ $((PIN % 0x1000)) -eq 0 ]]; then
        printf 'Current PIN: %06x\n' "$PIN"
    fi
    PIN=$((PIN + 1))
done

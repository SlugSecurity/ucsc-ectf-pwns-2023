#!/usr/bin/env python3

import json
import sys
import time
import serial

UNLOCK = b'\xaa\x0a'

ser = serial.Serial(sys.argv[1], 115200, inter_byte_timeout=0, stopbits=2, timeout=0.1)
esp32 = serial.Serial(sys.argv[2], 115200, inter_byte_timeout=0)

f = open("replay.json")

replays = json.load(f)

challenge = ''
while challenge not in replays.keys():
    response = ser.read(50).hex()
    if response in replays.keys():
        ser.write(bytes.fromhex(replays[response]))

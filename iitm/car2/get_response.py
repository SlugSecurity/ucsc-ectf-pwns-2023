#!/usr/bin/env python3

import json
import sys
import time
import serial

ser = serial.Serial(sys.argv[1], 115200, inter_byte_timeout=0, stopbits=2, timeout=0.1)
esp32 = serial.Serial(sys.argv[2], 115200, inter_byte_timeout=0)

cf = open("challenges.txt")
of = open("replay.json", "w")

replay = {}

for i in cf.readlines():
    response = ''
    while response == '':
        ser.reset_input_buffer()
        key = b'\x00\x00'
        while key != b'\xaa\x0a':
            esp32.write(b"l0r\n")
            time.sleep(0.01)
            esp32.write(b"h0r\n")
            time.sleep(0.1)
            esp32.write(b"l01\n")
            time.sleep(0.01)
            esp32.write(b"h01\n")
            key = ser.read(2)
            print(key.hex())
        ser.write(bytes.fromhex(i))
        response = ser.read(57)
        if response != '':
            replay[i.strip()] = response.hex()

of.write(json.dumps(replay))

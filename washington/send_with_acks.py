#!/usr/bin/python

from pwn import *
import sys

BYTES_PER_ACK = 8
ACK_MESSAGE = b"A"
HEADER = b"0ops"

io = serialtube(sys.argv[1], convert_newlines=False)

with open(sys.argv[2], "rb") as f:
    header = f.read(len(HEADER))
    io.send(header)

    # Received an ACK every 8th byte, starting after sending byte 1
    first = f.read(1)
    io.send(first)
    io.recv(1)

    while True:
        to_send = f.read(BYTES_PER_ACK)
        if not to_send:
            break
        io.send(to_send)
        # Only receive an ACK if we sent 8 bytes
        if len(to_send) == BYTES_PER_ACK:
            io.recv(1)

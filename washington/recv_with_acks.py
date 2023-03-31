#!/usr/bin/python

from pwn import *
import sys

BYTES_PER_ACK = 8
ACK_MESSAGE = b"A"
HEADER = b"0ops"

io = serialtube(sys.argv[1], convert_newlines=False)

header = io.recv(len(HEADER))
sys.stdout.buffer.write(header)
sys.stdout.buffer.flush()

# Send an ACK every 8th byte, starting after receiving byte 1
first = io.recv(1)
sys.stdout.buffer.write(first)
sys.stdout.buffer.flush()
io.send(ACK_MESSAGE)

while True:
    received = io.recv(BYTES_PER_ACK)
    sys.stdout.buffer.write(received)
    sys.stdout.buffer.flush()
    # Only send an ACK if we received 8 bytes
    if len(received) == BYTES_PER_ACK:
        io.send(ACK_MESSAGE)

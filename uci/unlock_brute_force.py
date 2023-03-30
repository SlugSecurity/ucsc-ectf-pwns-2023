#!/usr/bin/python3

from pwn import *
import random
import sys

UNLOCK_MAGIC = 0x56
ATTEMPT_COUNT = 1000000

io = serialtube(sys.argv[1], convert_newlines=False)

message_len = 16

for i in range(ATTEMPT_COUNT):
    received_message = io.recv()
    if received_message[1] == 0:
        print(f"Tried {i + 1} times")
    payload = p8(UNLOCK_MAGIC) + p8(message_len) + random.randbytes(message_len)
    io.send(payload)

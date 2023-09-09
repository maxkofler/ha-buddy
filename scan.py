'''Scan available addresses'''
#!python3

import sys
import serial

from python.frame import Frame, frame_decode, ExpectedBytesCountError

if len(sys.argv) <= 3:
    print("Usage: <serial port> <start address> <end address>")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
ser.timeout = 1

START_ADDR = int(sys.argv[2], 0)
END_ADDR = int(sys.argv[3], 0)

ser.timeout = 0.01

for addr in range(START_ADDR, END_ADDR):
    print(f"{hex(addr)}", end="\r", flush=True)

    frame = Frame(0x0000, addr, 0x0000, bytes([]))
    ser.write(frame.to_bytes())
    try:
        rec = frame_decode(ser)
    except ExpectedBytesCountError:
        pass

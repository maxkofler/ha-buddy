'''Test script'''
#!python3

import sys
import time

import serial

from python.frame import Frame, frame_decode, ExpectedBytesCountError

if len(sys.argv) <= 2:
    print("Usage: <serial port> <address>")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
time.sleep(2)
ser.timeout = 1

CLIENT_ADDR = int(sys.argv[2], 0)
print(f"Talking to {hex(CLIENT_ADDR)}")

def get_frame(serial_port: serial.Serial, cmd: int, payload: bytes):
    '''Sond'''
    send_frame = Frame(0x0000, CLIENT_ADDR, cmd, payload)
    serial_port.write(send_frame.to_bytes())
    return frame_decode(serial_port)

num: int = 0
try:
    num = int.from_bytes(get_frame(ser, 0x0200, bytes()).payload, byteorder="little")
except ExpectedBytesCountError:
    print(f"Client {hex(CLIENT_ADDR)} seems offline / not available")
    exit(0)
print(f"{num} available switches")
print("")

for i in range(0, num):
    s_id_bytes = i.to_bytes(4, byteorder="little")

    unique_id = get_frame(ser, 0x0202, s_id_bytes).payload.decode()
    name = get_frame(ser, 0x0204, s_id_bytes).payload.decode()
    value = bool(get_frame(ser, 0x0206, s_id_bytes).payload[0])

    print(f"Switch {i}:")
    print(f"    name:                       {name}")
    print(f"    unique_id:                  {unique_id}")
    print(f"    value:                      {value}")

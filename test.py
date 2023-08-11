import serial
import sys

import time

from crc import Calculator, Crc32
crc_calculator = Calculator(Crc32.BZIP2)

if len(sys.argv) <= 1:
    print("Need tty!")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
ser.timeout = 10

def calc_crc(data: bytes) -> bytes:
    global crc_calculator
    checksum = crc_calculator.checksum(data)
    return checksum.to_bytes(4, byteorder="little")

def create_msg(data: [int]) -> bytes:
    data = bytes(data)
    crc = bytearray(calc_crc(data))
    return data + crc


print("Waiting for slave to come online...")
time.sleep(1)

print("Go")

# Wrong frame (additional byte)
ser.write(bytes([0x00]))
ser.write(create_msg([0x00, 0x01, 0x02, 0x00, 0x02]))

time.sleep(2)

while True:
    ser.write(create_msg([0x00, 0x10, 0x02, 0x00, 0xfe]))
    ser.read(8)
    time.sleep(0.25)

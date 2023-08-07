import serial
import sys

import time

if len(sys.argv) <= 1:
    print("Need tty!")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
ser.timeout = 10

reset_data = bytes([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])

print("Waiting for initialization...")
data = ser.read(10)
if (data != reset_data):
    print("Initialization data was wrong!")
    exit(-1)

# Resetting
print("Resetting with \"{}\"".format(reset_data))
ser.write(reset_data)
ser.flush()

print("Waiting for initialization...")
data = ser.read(10)
print("Data: {}".format(data))
if (data != reset_data):
    print("Initialization data was wrong!")
    exit(-1)

print("Sending echo...")
ser.write(bytes([0xfe]))
data = ser.read(1)
if (data[0] != 0xfe):
    print("Echo failed")
    exit(-1)

print("Echo done")

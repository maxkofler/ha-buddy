'''Test script'''
#!python3

import sys
import time

import struct
import serial

from python.frame import Frame, frame_decode, ExpectedBytesCountError

if len(sys.argv) <= 2:
    print("Usage: <serial port> <address>")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
ser.timeout = 1
print("Waiting for device to reset...")
time.sleep(5)

CLIENT_ADDR = int(sys.argv[2], 0)
print(f"Talking to {hex(CLIENT_ADDR)}")

def get_frame(serial_port: serial.Serial, cmd: int, payload: bytes):
    '''Sond'''
    send_frame = Frame(0x0000, CLIENT_ADDR, cmd, payload)
    serial_port.write(send_frame.to_bytes())
    return frame_decode(serial_port)

num_sensors: int = 0
try:
    num_sensors = int.from_bytes(get_frame(ser, 0x0100, bytes()).payload, byteorder="little")
except ExpectedBytesCountError:
    print(f"Client {hex(CLIENT_ADDR)} seems offline / not available")
    exit(0)
print(f"{num_sensors} available sensors")
print("")

for i in range(0, num_sensors):
    s_id_bytes = i.to_bytes(4, byteorder="little")

    unique_id = get_frame(ser, 0x0102, s_id_bytes).payload.decode()
    native_unit_of_measurement = get_frame(ser, 0x0104, s_id_bytes).payload.decode()
    device_class = get_frame(ser, 0x0106, s_id_bytes).payload.decode()
    state_class = get_frame(ser, 0x0108, s_id_bytes).payload.decode()
    name = get_frame(ser, 0x0110, s_id_bytes).payload.decode()
    value = get_frame(ser, 0x0112, s_id_bytes).payload

    if value[0] == 2:
        value = struct.unpack("f", value[1:5])[0]
    elif value[0] == 1:
        value = int.from_bytes(value[1:5], byteorder="little")
    elif value[0] == 0:
        value = value[1:].decode()

    print(f"Sensor {i}:")
    print(f"    name:                       {name}")
    print(f"    unique_id:                  {unique_id}")
    print(f"    value:                      {value}")
    print(f"    native_unit_of_measurement: {native_unit_of_measurement}")
    print(f"    device_class:               {device_class}")
    print(f"    state_class:                {state_class}")

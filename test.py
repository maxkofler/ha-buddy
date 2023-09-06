'''Test script'''
#!python3
import time
import sys
import serial

from crc import Calculator, Crc32, Crc8
crc_calculator = Calculator(Crc32.BZIP2)

class Frame:
    '''A frame in the datalink layer'''

    START_BYTE_0 = 0xaa
    START_BYTE_1 = 0x55

    def __init__(self, src: int, dst: int, payload: bytes):
        '''Create new frame'''

        self.src = src
        self.dst = dst
        self.payload = payload

    def write(self, s_port: serial.Serial):
        '''Write out the frame'''

        frame_bytes = bytearray()

        frame_bytes += self.START_BYTE_0.to_bytes(1, byteorder="little")
        frame_bytes += self.START_BYTE_1.to_bytes(1, byteorder="little")
        frame_bytes += self.src.to_bytes(2, byteorder="little")
        frame_bytes += self.dst.to_bytes(2, byteorder="little")
        frame_bytes += len(self.payload).to_bytes(1, byteorder="little")

        header_calculator = Calculator(Crc8.BLUETOOTH)
        header_crc = header_calculator.checksum(frame_bytes)

        frame_bytes += header_crc.to_bytes(1, byteorder="little")
        #frame_bytes.append(self.payload)
        frame_bytes += self.payload

        frame_calculator = Calculator(Crc32.BZIP2)
        frame_crc = frame_calculator.checksum(frame_bytes)

        frame_bytes += frame_crc.to_bytes(4, byteorder="little")

        s_port.write(frame_bytes)

if len(sys.argv) <= 1:
    print("Need tty!")
    exit(-1)

ser = serial.Serial(sys.argv[1], baudrate=57600)
ser.timeout = 1

while True:
    frame = Frame(0x0000, 0x1000, bytes([0x00, 0xde]))
    frame.write(ser)
    rx = ser.read(13)
    print(f"Received: {rx}({len(rx)} bytes)")
    time.sleep(1)

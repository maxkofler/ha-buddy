"""The DataLink layer"""

import serial
import logging

LOGGER = logging.getLogger("ha_buddy")

from crc import Calculator, Crc8

START_BYTE_0 = 0xAA
START_BYTE_1 = 0x55


class Frame:
    """A frame in the datalink layer"""

    def __init__(self, src: int, dst: int, cmd: int, payload: bytes) -> None:
        """Create new frame"""

        self.src = src
        self.dst = dst
        self.cmd = cmd
        self.payload = payload

    def to_bytes(self) -> bytes:
        """Generate a byte array from this frame for transmission"""

        frame_bytes = bytearray()

        frame_bytes += START_BYTE_0.to_bytes(1, byteorder="little")
        frame_bytes += START_BYTE_1.to_bytes(1, byteorder="little")
        frame_bytes += self.src.to_bytes(2, byteorder="little")
        frame_bytes += self.dst.to_bytes(2, byteorder="little")
        frame_bytes += self.cmd.to_bytes(2, byteorder="little")
        frame_bytes += len(self.payload).to_bytes(1, byteorder="little")

        header_calculator = Calculator(Crc8.AUTOSAR)
        header_crc = header_calculator.checksum(frame_bytes)

        frame_bytes += header_crc.to_bytes(1, byteorder="little")
        frame_bytes += self.payload

        frame_calculator = Calculator(Crc8.AUTOSAR)
        frame_crc = frame_calculator.checksum(frame_bytes)

        frame_bytes += frame_crc.to_bytes(1, byteorder="little")

        return bytes(frame_bytes)


class ExpectedBytesCountError(Exception):
    """
    An error that gets raised if the expected amount of bytes
    wasn't fulfilled
    """

    def __init__(self, expected: int, got: int, reason: str) -> None:
        self.expected = expected
        self.got = got
        self.message = f"Expected {self.expected} bytes {reason}, got {self.got}"
        super().__init__(self.message)


class StartBytesError(Exception):
    """
    An error that gets raised if the start bytes weren't the first bytes
    """

    def __init__(self, got: bytes) -> None:
        self.got = got
        self.message = f"Expected start bytes, got {hex(self.got)}"
        super().__init__(self.message)


class HeaderCRCError(Exception):
    """
    An error that gets raised if the header crc did not match
    """

    def __init__(self, expected: int, got: int) -> None:
        self.expected = expected
        self.got = got
        self.message = f"Expected header crc {hex(self.expected)}, got {hex(self.got)}"
        super().__init__(self.message)


class FrameCRCError(Exception):
    """
    An error that gets raised if the frame crc did not match
    """

    def __init__(self, expected: int, got: int) -> None:
        self.expected = expected
        self.got = got
        self.message = f"Expected frame crc {hex(self.expected)}, got {hex(self.got)}"
        super().__init__(self.message)


def rec_bytes(count: int, ser: serial.Serial, reason: str) -> bytes:
    """Receive n bytes from the serial instance"""

    in_bytes = ser.read(count)

    if not len(in_bytes) == count:
        raise ExpectedBytesCountError(count, len(in_bytes), reason)

    return in_bytes


def frame_decode(ser: serial.Serial) -> Frame:
    """Decode a frame from a serial interface"""

    start_bytes = bytes([START_BYTE_0, START_BYTE_1])

    rec_start_bytes = rec_bytes(2, ser, "for start bytes")
    if not rec_start_bytes == start_bytes:
        raise StartBytesError(rec_start_bytes)

    received_bytes = bytearray(start_bytes)

    b_src = rec_bytes(2, ser, "for src")
    received_bytes += b_src
    src = int.from_bytes(b_src, byteorder="little")

    b_dst = rec_bytes(2, ser, "for dst")
    received_bytes += b_dst
    dst = int.from_bytes(b_dst, byteorder="little")

    b_cmd = rec_bytes(2, ser, "for cmd")
    received_bytes += b_cmd
    cmd = int.from_bytes(b_cmd, byteorder="little")

    b_payload_len = rec_bytes(1, ser, "for payload len")
    received_bytes += b_payload_len
    payload_len = int.from_bytes(b_payload_len, byteorder="little")

    b_header_crc = rec_bytes(1, ser, "for header crc")
    header_crc = int.from_bytes(b_header_crc, byteorder="little")

    header_calculator = Calculator(Crc8.AUTOSAR)
    calculated_header_crc = header_calculator.checksum(received_bytes)

    if not calculated_header_crc == header_crc:
        raise HeaderCRCError(calculated_header_crc, header_crc)
    received_bytes += b_header_crc

    payload = rec_bytes(payload_len, ser, "for payload")
    received_bytes += payload

    b_frame_crc = rec_bytes(1, ser, "for frame crc")
    frame_crc = int.from_bytes(b_frame_crc, byteorder="little")

    frame_calculator = Calculator(Crc8.AUTOSAR)
    calculated_frame_crc = frame_calculator.checksum(received_bytes)

    if not calculated_frame_crc == frame_crc:
        raise FrameCRCError(calculated_frame_crc, frame_crc)

    return Frame(src, dst, cmd, payload)


def exec_command(ser: serial.Serial, out_frame: Frame) -> bytes:
    """
    Tries to execute a command and return the payload.
    If the client does not return the right command, it gets thrown away
    """

    ser.write(out_frame.to_bytes())

    while True:
        in_frame = frame_decode(ser)

        if in_frame.cmd == out_frame.cmd + 1:
            return in_frame.payload

        LOGGER.error(
            f"Invalid response command: Expected {hex(out_frame.cmd+1)}, got {hex(in_frame.cmd)}"
        )

import serial
import logging
import time

from .frame import Frame, frame_decode, ExpectedBytesCountError, exec_command

LOGGER = logging.getLogger("ha_buddy")


class BuddyConnection:
    def __init__(self, port: str, domain: str) -> None:
        self._port = port
        self._domain = domain
        self._ser = None
        self.devices = None

    def connect_and_scan(self) -> bool:
        from .device import Device

        try:
            self._ser = serial.Serial(self._port, baudrate=57600)
        except:
            LOGGER.error("Failed to open serial port!")
            return False

        LOGGER.info("Waiting 5 seconds for all devices to come up")
        time.sleep(5)

        LOGGER.info("Scanning for devices")

        self.devices = []
        for addr in range(0x1000, 0x1010):
            self._ser.timeout = 0.2
            frame = Frame(0x0000, addr, 0x0000, bytes([]))
            self._ser.write(frame.to_bytes())

            try:
                rec = frame_decode(self._ser)
            except ExpectedBytesCountError:
                continue

            LOGGER.info(f"Device {hex(addr)} is online ({rec.payload})!")

            self.devices.append(Device(self._domain, addr, self))

        return True

    def get_payload(self, client_addr: int, cmd: int, payload: bytes) -> bytes:
        send_frame = Frame(0x0000, client_addr, cmd, payload)
        return exec_command(self._ser, send_frame)

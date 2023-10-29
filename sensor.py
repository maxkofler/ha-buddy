"""Platform for sensor integration."""
from __future__ import annotations

import serial
import struct
import time

from homeassistant.components.sensor import (
    SensorEntity,
)
from homeassistant.helpers import device_registry as dr

from .python.frame import Frame, frame_decode, ExpectedBytesCountError, exec_command
import logging

from . import DOMAIN
from .const import *

LOGGER = logging.getLogger(DOMAIN)


async def async_setup_entry(hass, config_entry, async_add_devices):
    """Setup sensor platform."""

    try:
        ser = serial.Serial("/dev/ttyUSB0", baudrate=57600)
    except:
        LOGGER.error("Failed to open serial port!")
        return

    LOGGER.info("Waiting 5 seconds for all devices to come up...")
    time.sleep(5)

    LOGGER.info("Scanning for devices")

    devices = []
    for addr in range(0x1000, 0x1010):
        ser.timeout = 0.2
        frame = Frame(0x0000, addr, 0x0000, bytes([]))
        ser.write(frame.to_bytes())

        try:
            rec = frame_decode(ser)
        except ExpectedBytesCountError:
            continue
        LOGGER.info(f"Device {hex(addr)} is online ({rec.payload})!")

        ser.timeout = 2
        device = Device(addr, ser)
        async_add_devices(device.get_sensors())
        devices.append(device)

    LOGGER.info("Done scanning!")


def get_payload(
    serial_port: serial.Serial, client_addr: int, cmd: int, payload: bytes
) -> bytes:
    send_frame = Frame(0x0000, client_addr, cmd, payload)
    return exec_command(serial_port, send_frame)


class Device:
    def __init__(self, addr: int, ser: serial.Serial) -> None:
        self._addr = addr
        self._ser = ser

        self._device_info = dr.DeviceInfo(
            identifiers={(DOMAIN, hex(self._addr))},
            name=f"Buddy {hex(self._addr)}",
            manufacturer="Max Kofler",
            model="buddy 1.0",
            sw_version="1.0",
        )

    def get_device_payload(self, cmd: int, payload: bytes):
        return get_payload(self._ser, self._addr, cmd, payload)

    def get_sensors(self) -> []:
        num_sensors = int.from_bytes(
            self.get_device_payload(CMD_SENSOR_DISCOVERY, bytes()),
            byteorder="little",
        )

        LOGGER.info(f"Device {hex(self._addr)} has {num_sensors} available sensors")

        sensors = []

        for i in range(0, num_sensors):
            sensors.append(BuddySensor(self, i))

        return sensors

    def device_info(self) -> dr.DeviceInfo:
        return self._device_info

    def addr(self) -> int:
        return self._addr


class BuddySensor(SensorEntity):
    """A HA Buddy sensor"""

    def __init__(self, device: Device, sensor_id: int) -> None:
        self._device = device
        self._sensor_id = sensor_id

        s_id = self._sensor_id.to_bytes(4, byteorder="little")

        LOGGER.info(
            f"Retrieving attributes for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}"
        )
        self._attr_name = self._device.get_device_payload(
            CMD_SENSOR_NAME, s_id
        ).decode()
        LOGGER.debug(
            f"  Name for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_name}"
        )

        unique_id = self._device.get_device_payload(CMD_SENSOR_UNIQUE_ID, s_id).decode()
        # unique_id = self._device.get_device_frame(0x0102, s_id).payload.decode()
        self._attr_unique_id = f"sensor.ha_buddy_{hex(self._device._addr)}_{unique_id}"
        self.entity_id = self._attr_unique_id
        LOGGER.info(
            f"Unique id for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_unique_id}"
        )

        self._attr_native_unit_of_measurement = self._device.get_device_payload(
            CMD_SENSOR_NATIVE_UNIT_OF_MEASUREMENT, s_id
        ).decode()
        LOGGER.debug(
            f"  Native unit of measurement for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_native_unit_of_measurement}"
        )

        self._attr_device_class = self._device.get_device_payload(
            CMD_SENSOR_DEVICE_CLASS, s_id
        ).decode()
        LOGGER.debug(
            f"  Device class for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_device_class}"
        )

        self._attr_state_class = self._device.get_device_payload(
            CMD_SENSOR_STATE_CLASS, s_id
        ).decode()
        LOGGER.debug(
            f"  State class for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_state_class}"
        )

        self._attr_native_value = self.get_value()
        LOGGER.debug(
            f"  Value for sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_native_value}"
        )

    def get_value(self) -> int | str | float:
        value = self._device.get_device_payload(
            CMD_SENSOR_VALUE, self._sensor_id.to_bytes(4, byteorder="little")
        )

        if value[0] == 2:
            value = struct.unpack("f", value[1:5])[0]
        elif value[0] == 1:
            value = int.from_bytes(value[1:5], byteorder="little")
        elif value[0] == 0:
            value = value[1:].decode()

        return value

    @property
    def device_info(self) -> dr.DeviceInfo:
        return self._device.device_info()

    def update(self) -> None:
        self._attr_native_value = self.get_value()
        LOGGER.debug(
            f"Value of sensor {hex(self._device.addr())}:{hex(self._sensor_id)}: {self._attr_native_value}"
        )

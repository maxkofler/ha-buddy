import logging

from homeassistant.helpers import device_registry as dr
from .entities.sensor import BuddySensor
from .entities.switch import BuddySwitch

LOGGER = logging.getLogger("ha_buddy")

CMD_SENSOR_DISCOVERY = 0x0100
CMD_SWITCH_DISCOVERY = 0x0200


class Device:
    def __init__(self, domain: str, addr: int, con) -> None:
        self._addr = addr
        self._con = con

        self._device_info = dr.DeviceInfo(
            identifiers={(domain, hex(self._addr))},
            name=f"Buddy {hex(self._addr)}",
            manufacturer="Max Kofler",
            model="buddy 1.0",
            sw_version="1.0",
        )

    def get_device_payload(self, cmd: int, payload: bytes):
        return self._con.get_payload(self._addr, cmd, payload)

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

    def get_switches(self) -> []:
        num_switchs = int.from_bytes(
            self.get_device_payload(CMD_SWITCH_DISCOVERY, bytes()),
            byteorder="little",
        )

        LOGGER.info(f"Device {hex(self._addr)} has {num_switchs} available switches")

        switches = []

        for i in range(0, num_switchs):
            switches.append(BuddySwitch(self, i))

        return switches

    def device_info(self) -> dr.DeviceInfo:
        return self._device_info

    def addr(self) -> int:
        return self._addr

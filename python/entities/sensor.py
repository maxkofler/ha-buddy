import struct
import logging

from homeassistant.helpers import device_registry as dr
from homeassistant.components.sensor import (
    SensorEntity,
)

LOGGER = logging.getLogger("ha_buddy")

CMD_SENSOR_UNIQUE_ID = 0x0102
CMD_SENSOR_NATIVE_UNIT_OF_MEASUREMENT = 0x0104
CMD_SENSOR_DEVICE_CLASS = 0x0106
CMD_SENSOR_STATE_CLASS = 0x0108
CMD_SENSOR_NAME = 0x0110
CMD_SENSOR_VALUE = 0x0112


class BuddySensor(SensorEntity):
    """A HA Buddy sensor"""

    def __init__(self, device, sensor_id: int) -> None:
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

    def get_value(self) -> None | int | str | float:
        value = self._device.get_device_payload(
            CMD_SENSOR_VALUE, self._sensor_id.to_bytes(4, byteorder="little")
        )

        if len(value) == 0:
            value = None
        elif value[0] == 2:
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

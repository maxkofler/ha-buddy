import struct
import logging

from homeassistant.helpers import device_registry as dr
from homeassistant.components.switch import (
    SwitchEntity,
)

LOGGER = logging.getLogger("ha_buddy")

CMD_SWITCH_UNIQUE_ID = 0x0202
CMD_SWITCH_NAME = 0x0204
CMD_SWITCH_STATE = 0x0206
CMD_SWITCH_EXEC = 0x0208

CMD_SWITCH_EXEC_TURN_OFF = 0
CMD_SWITCH_EXEC_TURN_ON = 1
CMD_SWITCH_EXEC_TOGGLE = 2


class BuddySwitch(SwitchEntity):
    """A HA Buddy switch"""

    def __init__(self, device, switch_id: int) -> None:
        self._device = device
        self._switch_id = switch_id
        self._is_on = False

        s_id = self._switch_id.to_bytes(4, byteorder="little")

        LOGGER.info(
            f"Retrieving attributes for switch {hex(self._device.addr())}:{hex(self._switch_id)}"
        )
        self._attr_name = self._device.get_device_payload(
            CMD_SWITCH_NAME, s_id
        ).decode()
        LOGGER.debug(
            f"  Name for switch {hex(self._device.addr())}:{hex(self._switch_id)}: {self._attr_name}"
        )

        unique_id = self._device.get_device_payload(CMD_SWITCH_UNIQUE_ID, s_id).decode()
        self._attr_unique_id = f"switch.ha_buddy_{hex(self._device._addr)}_{unique_id}"
        self.entity_id = self._attr_unique_id
        LOGGER.info(
            f"Unique id for switch {hex(self._device.addr())}:{hex(self._switch_id)}: {self._attr_unique_id}"
        )

    def turn_on(self, **kwargs):
        payload = bytearray(self._switch_id.to_bytes(4, byteorder="little")) + bytes(
            [CMD_SWITCH_EXEC_TURN_ON]
        )

        value = self._device.get_device_payload(CMD_SWITCH_EXEC, payload)

        self._is_on = value[0] != 0

    def turn_off(self, **kwargs):
        payload = bytearray(self._switch_id.to_bytes(4, byteorder="little")) + bytes(
            [CMD_SWITCH_EXEC_TURN_OFF]
        )

        value = self._device.get_device_payload(CMD_SWITCH_EXEC, payload)

        self._is_on = value[0] != 0

    @property
    def device_info(self) -> dr.DeviceInfo:
        return self._device.device_info()

    @property
    def is_on(self) -> bool:
        return self._is_on

    def update(self) -> None:
        value = self._device.get_device_payload(
            CMD_SWITCH_STATE, self._switch_id.to_bytes(4, byteorder="little")
        )

        self._is_on = value[0] != 0

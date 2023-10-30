"""Platform for sensor integration."""
from __future__ import annotations

from homeassistant.core import HomeAssistant
from homeassistant.helpers import device_registry as dr

import logging

from . import DOMAIN
from .const import *

LOGGER = logging.getLogger(DOMAIN)


async def async_setup_entry(hass: HomeAssistant, config_entry, async_add_devices):
    """Setup sensor platform."""

    if hass.data[DOMAIN] is None:
        LOGGER.error("Can't setup ha_buddy sensors: NO CONNECTION")
        return

    LOGGER.info("Setting up sensors")

    for device in hass.data[DOMAIN].devices:
        async_add_devices(device.get_sensors())

    LOGGER.info("Done setting up sensors")

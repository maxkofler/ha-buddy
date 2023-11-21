"""Platform for switch integration."""
from __future__ import annotations

from homeassistant.core import HomeAssistant
from homeassistant.helpers import device_registry as dr

import logging

from . import DOMAIN
from .const import *

LOGGER = logging.getLogger(DOMAIN)

async def async_setup_entry(
    hass: HomeAssistant,
    config_entry: ConfigEntry,
    async_add_entities: AddEntitiesCallback,
) -> None:
    """Setup switch platform."""

    if hass.data[DOMAIN] is None:
        LOGGER.error("Can't setup ha_buddy switches: NO CONNECTION")
        return

    LOGGER.info("Setting up switches")

    for device in hass.data[DOMAIN].devices:
        async_add_entities(device.get_switches())

    LOGGER.info("Done setting up switches")

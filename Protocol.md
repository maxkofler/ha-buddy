# HA-Buddy protocol

Master-Slave based

## Initialization

Slave sends following data string to the master once it is ready:

```
[0xff; 10]
```

## Reset

The slave waits for the following sequence:

```
[0xff; 10]
```

It is possible to send more that `10` bytes, the slave will discard them anyway

# Commands:

- `0xfe` - Echo



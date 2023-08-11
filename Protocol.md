# HA-Buddy protocol

Every frame is a protocol message. The maximum message length is `256` bytes.

# Message structure:

`[command: u16; ...]`

The rest of the message body is optional for every command.

# Commands:

### `0x00` - Echo

Echo back a specific value provided by the master

**Request**

`[0x00; 0x<data>]`

**Response:**

`[0x<data>]`

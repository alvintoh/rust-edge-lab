# Modbus notes

Source: MODBUS Application Protocol Specification V1.1b3 (2012),
<https://www.modbus.org/file/secure/modbusprotocolspecification.pdf>

## Four tables (spec 4.3)

| Table | Size | Access | Example |
|---|---|---|---|
| Discrete Inputs | 1 bit | read-only | alarm flag |
| Coils | 1 bit | read-write | contactor on/off |
| Input Registers | 16-bit word | read-only | measured voltage |
| Holding Registers | 16-bit word | read-write | charge-power limit |

## Function codes (spec 6.1-6.6)

| Code | Name |
|---|---|
| 01 (0x01) | Read Coils |
| 02 (0x02) | Read Discrete Inputs |
| 03 (0x03) | Read Holding Registers |
| 04 (0x04) | Read Input Registers |
| 05 (0x05) | Write Single Coil |
| 06 (0x06) | Write Single Register (a holding register) |

One READ per table, one WRITE per writable table. Read-only tables have no write code. The spec
gives no reason for the numbering order, so treat the codes as fixed labels, like HTTP 404.

## Error replies (spec 4.1)

An exception reply sets the top bit, so the code is exactly 0x80 higher. 0x83 means "your 0x03
failed".

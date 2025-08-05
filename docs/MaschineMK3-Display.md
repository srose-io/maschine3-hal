# Native Instruments Maschine MK3 Display Protocol

The Maschine MK3 Displays are listening on `Interface #5`, `Endpoint 0x04`.

## Display Information

Both displays are 480x272 pixels each.

## RGB565 Pixel Format

The protocol uses the RGB565 pixel format, which is basically 24 bit RGB converted to
16 bits:

- Red uses 5 bits
- Green uses 6 bits
- Blue uses 5 bits

There is a possibility that some value might be transparent, but this needs to be verified.

## Protocol Overview

<table style="whitespace: nowrap;">
    <tr>
        <td colspan="4" style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 32 bytes → </td>
    </tr>
    <tr>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 16 bytes → </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 16 bytes → </td>
    </tr>
    <tr>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 8 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 8 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 8 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 8 bytes → </td>
    </tr>
    <tr valign="top">
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header Part 1</b>
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header Part 2</b>
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>…</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>…</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>…</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>…</b>
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command or Data</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>End of Transmission Command</b>
        </td>
    </tr>
</table>

## Header Format (16 bytes total) - CORRECTED

**⚠️ IMPORTANT:** This documentation has been corrected based on working implementations. The coordinates are at byte offsets 8-15, not 16+ as originally documented.

<table style="whitespace: nowrap;">
    <tr>
        <td colspan="8" style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 16 bytes → </td>
    </tr>
    <tr>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 2 bytes → </td>
    </tr>
    <tr>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 0-1</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 2-3</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 4-5</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 6-7</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 8-9</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 10-11</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 12-13</b> </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> <b>Offset 14-15</b> </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Packet Type</b><br/><br/>
            [0] = 0x84<br/>
            [1] = 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Display ID</b><br/><br/>
            [2] = Display ID<br/>
            0x00: Left Display<br/>
            0x01: Right Display<br/>
            [3] = 0x60
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Reserved</b><br/><br/>
            [4] = 0x00<br/>
            [5] = 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Reserved</b><br/><br/>
            [6] = 0x00<br/>
            [7] = 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>X Start</b><br/><br/>
            [8] = X MSB<br/>
            [9] = X LSB<br/>
            (Big-endian)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Y Start</b><br/><br/>
            [10] = Y MSB<br/>
            [11] = Y LSB<br/>
            (Big-endian)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Width</b><br/><br/>
            [12] = Width MSB<br/>
            [13] = Width LSB<br/>
            (Big-endian)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Height</b><br/><br/>
            [14] = Height MSB<br/>
            [15] = Height LSB<br/>
            (Big-endian)
        </td>
    </tr>
</table>

## Working Implementation Notes

Based on successful reverse engineering and working implementations, the following has been confirmed:

### Practical Packet Structure

For full-screen rendering, the most effective approach is **row-by-row rendering**:

- **Packet Size:** 988 bytes total
  - Header: 16 bytes
  - Transmit Command: 4 bytes  
  - Pixel Data: 960 bytes (480 pixels × 2 bytes RGB565)
  - Blit Command: 4 bytes (0x03 0x00 0x00 0x00)
  - End Command: 4 bytes (0x40 0x00 0x00 0x00)

- **Full Screen:** 272 packets (one per row) × 480 pixels each
- **Coordinates:** Use (0, row_number, 480, 1) for each row

### RGB565 Encoding

Confirmed RGB565 format (little-endian on USB):
```
RGB565 = ((red & 0x1F) << 11) | ((green & 0x3F) << 5) | (blue & 0x1F)
packet[offset] = (RGB565 & 0xFF) as u8;     // LSB first
packet[offset+1] = (RGB565 >> 8) as u8;    // MSB second
```

### USB Communication

- **Interface:** 5 (WinUSB driver required on Windows)
- **Endpoint:** 0x04 (bulk transfer)
- **Vendor ID:** 0x17CC
- **Product ID:** 0x1600

### Commands

Each command consists of 4 bytes with an optional multiple of 4 bytes data:

<table style="whitespace: nowrap;">
    <tr>
        <td colspan="8" style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 8 bytes → </td>
    </tr>
    <tr>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> ← 1 byte → </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command Code</b><br/><br/>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command Parameter 1</b><br/><br/>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command Parameter 2</b><br/><br/>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Command Parameter 3</b><br/><br/>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data 1</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data 2</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data 3</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data 4</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data n+1</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data n+2</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data n+3</b><br/><br/>
            Optional
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Data n+4</b><br/><br/>
            Optional
        </td>
    </tr>
</table>

<table style="whitespace: nowrap;">
    <tr>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Command Code </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Description </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Parameter 1 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Parameter 2 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Parameter 3 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Data 1 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Data 2 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Data 3 </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;"> Data 4 </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>0x00</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Transmits the (n)*2 given pixels</b><br/><br/>
            Advances pixel cursor<br/>
            You may specify any number of pixels*2,<br/>
            which need to be included as data.
        </td>
        <td colspan="3" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>24 bit integer</b><br/>
            MSB in Parameter 1<br/>
            LSB in Parameter 3
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>First Pixel</b><br/>
            RGB565 Format
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Second Pixel</b><br/>
            RGB565 Format
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>0x01</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Repeat the two given pixels (n) times</b><br/><br/>
            Advances pixel cursor
        </td>
        <td colspan="3" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Number of (n) repetitions</b><br/>
            24 bit integer<br/>
            MSB in Parameter 1<br/>
            LSB in Parameter 3<br/><br/>
            Example: If you transmit white,black 5 times, you end up with:<br/>
            white,black,white,black,white,black,white,black,white,black
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>First Pixel</b><br/>
            RGB565 Format
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Second Pixel</b><br/>
            RGB565 Format
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>0x03</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Unknown, probably to blit the drawn data </b><br/><br/>
        </td>
        <td colspan="3" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Unknown</b><br/>
            Must be 0x00 each<br/>
            Mandatory
        </td>
        <td colspan="4" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Not used</b><br/>
            Must not be present
        </td>
    </tr>
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>0x40</b>
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>End of data</b><br/><br/>
        </td>
        <td colspan="3" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Not used</b><br/>
            Mandatory<br/>
            Must be 0x00 each
        </td>
        <td colspan="4" style="white-space:nowrap;font-family:monospace;text-align: center;">
        <b>Not used</b><br/>
        Must not be present
        </td>
        </tr>
        </table>

## Example: Single Row Packet

Here's a complete example of a working packet that renders one row (480 pixels) to the left display:

```
Offset  Value   Description
------  -----   -----------
0x00    0x84    Packet type
0x01    0x00    Reserved
0x02    0x00    Display ID (0=left, 1=right)  
0x03    0x60    Always 0x60
0x04    0x00    Reserved
0x05    0x00    Reserved
0x06    0x00    Reserved
0x07    0x00    Reserved
0x08    0x00    X start (MSB) = 0
0x09    0x00    X start (LSB) = 0
0x0A    0x00    Y start (MSB) = 0 (for row 0)
0x0B    0x00    Y start (LSB) = 0
0x0C    0x01    Width (MSB) = 480
0x0D    0xE0    Width (LSB) = 480  
0x0E    0x00    Height (MSB) = 1
0x0F    0x01    Height (LSB) = 1

0x10    0x00    Transmit command
0x11    0x00    Pixel count (MSB) = 240 (480/2)
0x12    0x00    Pixel count (MID) = 240
0x13    0xF0    Pixel count (LSB) = 240

0x14    ...     960 bytes of RGB565 pixel data
        ...     (480 pixels × 2 bytes each)

0x3D0   0x03    Blit command
0x3D1   0x00    Reserved
0x3D2   0x00    Reserved  
0x3D3   0x00    Reserved

0x3D4   0x40    End transmission command
0x3D5   0x00    Reserved
0x3D6   0x00    Reserved
0x3D7   0x00    Reserved
```

**Total packet size: 988 bytes (0x3DC)**

## Implementation References

Working implementations have been successfully created that render full-screen graphics to both displays. Key examples:

- **Raw Loader Example:** Parses and modifies existing Wireshark captures with corrected coordinate parsing
- **Working Display Example:** Creates complete row-by-row rendering with gradient and checkerboard patterns

Both examples confirm the corrected header format and demonstrate full 480×272 pixel rendering capability.
 


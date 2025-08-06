# Native Instruments Maschine MK3 Display Protocol

> **Attribution**: This documentation is adapted from [Drachenkaetzchen/cabl](https://github.com/Drachenkaetzchen/cabl/tree/develop/doc/hardware/maschine-mk3) and has been significantly enhanced and expanded with additional research and implementation details.

The Maschine MK3 Displays are listening on `Interface #5`, `Endpoint 0x04`.

## Display Information

Both displays are 480x272 pixels each.

## RGB565X Pixel Format (CORRECTED)

The protocol uses a **custom RGB565X pixel format** with unusual bit packing:

**Bit Layout:** `GGGB BBBB RRRR RGGG` (16 bits)

- **Green:** 6 bits split (3 MSB + 3 LSB)
- **Blue:** 5 bits (middle)
- **Red:** 5 bits split (4 middle + 1 offset)

**Channel Rotation:** The device has rotated color channels:

- Input RED → Display BLUE
- Input GREEN → Display RED
- Input BLUE → Display GREEN

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

### Optimal Packet Structure

For maximum performance, use **single large packet** for entire frame:

- **Packet Size:** 261,148 bytes total

  - Header: 16 bytes
  - Transmit Command: 4 bytes
  - Pixel Data: 261,120 bytes (130,560 pixels × 2 bytes RGB565X)
  - Blit Command: 4 bytes (0x03 0x00 0x00 0x00)
  - End Command: 4 bytes (0x40 0x00 0x00 0x00)

- **Performance:** ~30 FPS achievable
- **Coordinates:** Use (0, 0, 480, 272) for full screen

**Note:** Row-by-row packets (272 separate transmissions) achieve only ~0.6 FPS due to USB overhead.

### RGB565X Encoding (CORRECTED)

Custom RGB565X format with channel rotation:

```rust
fn rgb565x(red: u8, green: u8, blue: u8) -> u16 {
    // Apply MK3 channel rotation
    let corrected_r = blue;  // Red channel gets blue input
    let corrected_g = red;   // Green channel gets red input
    let corrected_b = green; // Blue channel gets green input

    // Pack as: GGGB BBBB RRRR RGGG
    let r4 = (corrected_r >> 4) as u16;     // Red high: 4 bits
    let r1 = (corrected_r >> 3) & 0x1;      // Red low: 1 bit
    let b5 = (corrected_b >> 3) as u16;     // Blue: 5 bits
    let g_high = (corrected_g >> 5) as u16; // Green high: 3 bits
    let g_low = (corrected_g >> 3) & 0x7;   // Green low: 3 bits

    (g_high << 13) | (b5 << 8) | (r4 << 4) | (r1 << 3) | g_low
}

// Store as little-endian
packet[offset] = (rgb565x & 0xFF) as u8;     // LSB first
packet[offset+1] = (rgb565x >> 8) as u8;    // MSB second
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
            <b>24 bit integer (pixel_count / 2)</b><br/>
            MSB in Parameter 1<br/>
            LSB in Parameter 3
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>First Pixel</b><br/>
            RGB565X Format
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Second Pixel</b><br/>
            RGB565X Format
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
            RGB565X Format
        </td>
        <td colspan="2" style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Second Pixel</b><br/>
            RGB565X Format
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

## Example: Full Screen Packet (Optimal)

Here's a complete example of an optimized packet that renders the entire display (480×272 pixels) to the left display:

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
0x0A    0x00    Y start (MSB) = 0
0x0B    0x00    Y start (LSB) = 0
0x0C    0x01    Width (MSB) = 480
0x0D    0xE0    Width (LSB) = 480
0x0E    0x01    Height (MSB) = 272
0x0F    0x10    Height (LSB) = 272

0x10    0x00    Transmit command
0x11    0x00    Half-pixel count (MSB) = 65,280 (130,560/2)
0x12    0xFF    Half-pixel count (MID) = 65,280
0x13    0x00    Half-pixel count (LSB) = 65,280

0x14    ...     261,120 bytes of RGB565X pixel data
        ...     (130,560 pixels × 2 bytes each)

0x3FB18 0x03    Blit command
0x3FB19 0x00    Reserved
0x3FB1A 0x00    Reserved
0x3FB1B 0x00    Reserved

0x3FB1C 0x40    End transmission command
0x3FB1D 0x00    Reserved
0x3FB1E 0x00    Reserved
0x3FB1F 0x00    Reserved
```

**Total packet size: 261,148 bytes (0x3FB20)**  
**Performance: ~30 FPS**

## Implementation References

Working implementations have been successfully created that render full-screen graphics to both displays. Key examples:

- **Single Packet Rendering:** Achieves ~30 FPS with optimized single large packet transmission for one screen
- **RGB565X Format:** Custom bit packing with channel rotation successfully reverse-engineered
- **Performance Comparison:** Single packet

All examples confirm the corrected RGB565X format, channel rotation, and optimal packet structure for maximum performance.

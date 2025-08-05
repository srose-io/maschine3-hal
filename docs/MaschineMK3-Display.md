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

## Header Part 1

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
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 1</b><br/><br/>
            Always 0x84
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 2</b><br/><br/>
            Always 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 3</b><br/><br/>
            Output Display:<br/><br/>
            0x00: Left Display<br/>
            0x01: Right Display
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 4</b><br/><br/>
            Always 0x60
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 5</b><br/><br/>
            Always 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 6</b><br/><br/>
            Always 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 7</b><br/><br/>
            Always 0x00
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 8</b><br/><br/>
            Always 0x00
        </td>
    </tr>
</table>

The table above lists the header format. Apart from `Header 3`, which specifies the output display,
all other values seem to be fixed in value.

## Header Part 2
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
    <tr valign="top">
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 9</b><br/><br/>
            X Start Address (MSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 10</b><br/><br/>
            X Start Address (LSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 11</b><br/><br/>
            Y Start Address (MSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 12</b><br/><br/>
            Y Start Address (LSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 13</b><br/><br/>
            Width (MSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 14</b><br/><br/>
            Width (LSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 15</b><br/><br/>
            Height (MSB)
        </td>
        <td style="white-space:nowrap;font-family:monospace;text-align: center;">
            <b>Header 16</b><br/><br/>
            Height (LSB)
        </td>
    </tr>
</table>

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



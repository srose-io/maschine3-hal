# MK3 Display Protocol - BREAKTHROUGH!

## Display Protocol Discovered!

### Simple Update Packet Format (32 bytes):
```
84 00 [DISP] 60 00 00 00 00 [Y_LO] [Y_HI] 01 00 00 [X] 00 [W] 01 00 00 [W] [COLOR_LO] [COLOR_HI] 00 00 03 00 00 00 40 00 [DISP] 00
```

Where:
- `DISP`: Display ID (00=left, 01=right)  
- `Y_LO Y_HI`: Y coordinate (little endian)
- `X`: X coordinate 
- `W`: Width in pixels
- `COLOR_LO COLOR_HI`: RGB565 color value
- `40 00 [DISP] 00`: Terminator with display ID

## Examples:

### Left Screen Progress Bar (Y=402, X=2, W=10, Color=0x81f8):
```
84 00 00 60 00 00 00 00 01 92 01 00 00 02 00 0a 01 00 00 0a f8 81 00 00 03 00 00 00 40 00 00 00
```

### Right Screen Progress Bar (Y=338, X=2, W=10, Color=0x81f8):  
```
84 00 01 60 00 00 00 00 00 52 01 00 00 02 00 0a 01 00 00 0a f8 81 00 00 03 00 00 00 40 00 01 00
```

## Key Findings:
1. **Display ID**: Byte 3 = 00 (left), 01 (right)
2. **RGB565 Colors**: 16-bit color values work!
3. **Simple RLE**: Fills W pixels with same color
4. **Consistent Terminator**: 0x40 command

## Frame 1915 - Display Data Transfer (2012 bytes)

### Header Analysis
```
84 00 00 60 00 00 00 00 00 7e 00 04 00 70 00 19 00 00 00 01
```

Breaking this down:
- `84 00` - Command identifier 
- `00 60` - Possibly related to display setup
- `00 00 00 00` - Padding/reserved
- `00 7e` - Unknown field
- `00 04` - Possibly related to endpoint 4
- `00 70 00 19` - Width/height? (0x70 = 112, 0x19 = 25)
- `00 00 00 01` - Display ID or command flag

### Data Pattern Analysis
The packet contains what appears to be RLE (Run Length Encoded) pixel data:

```
08 61 bd f7 01 00 00 35 ff ff ff ff 00 00 00 03
```

This could be interpreted as:
- `08 61 bd f7` - 4-byte color value (possibly RGBA or special encoding)
- `01 00 00 35` - Run length (0x35 = 53 pixels)
- `ff ff ff ff` - Next color value
- `00 00 00 03` - Next run length (3 pixels)

### Key Observations
1. **Packet Size**: 2012 bytes - much larger than previous documentation suggested
2. **RLE Encoding**: Data appears to use run-length encoding rather than raw pixel data
3. **32-bit Colors**: Color values appear to be 4 bytes each, not 2-byte RGB565
4. **Command Structure**: Header + RLE data + possible terminator

## Packet Ending
```
03 00 00 00 40 00 00 00
```
- Ends with `40 00 00 00` - likely the terminator command (0x40)

## Next Steps
1. Analyze the RLE pattern more thoroughly
2. Compare with other display captures to verify pattern
3. Test implementing this RLE format

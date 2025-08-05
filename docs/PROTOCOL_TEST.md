# MK3 Protocol Test - Simple Packet Format

## Overview

This test validates the **newly discovered** simple 32-byte packet format for the Maschine MK3 displays, reverse-engineered from Wireshark USB captures.

## Running the Test

```bash
cargo run --example protocol_test_simple
```

## What the Test Does

The test sends 4 different colored rectangles to both displays:

1. **Red rectangle** on left screen (Y=100, X=50, W=20)
2. **Green rectangle** on right screen (Y=150, X=100, W=30) 
3. **Blue rectangle** on left screen (Y=200, X=200, W=50)
4. **Yellow progress bar** on right screen (exact replica from capture)

## Expected Results

✅ **Success**: You should see colored horizontal lines appear on the displays
❌ **Failure**: Displays remain unchanged (black or showing original content)

## Packet Format Discovered

```
84 00 [DISP] 60 00 00 00 00 [Y_LO] [Y_HI] 01 00 00 [X] 00 [W] 01 00 00 [W] [COLOR_LO] [COLOR_HI] 00 00 03 00 00 00 40 00 [DISP] 00
```

### Field Breakdown:
- `DISP`: Display ID (00=left, 01=right)
- `Y_LO Y_HI`: Y coordinate (little endian 16-bit)  
- `X`: X coordinate (8-bit)
- `W`: Width in pixels (8-bit)
- `COLOR_LO COLOR_HI`: RGB565 color (little endian)
- Final bytes: RLE count and terminator

## Key Discoveries

🎯 **Display Selection**: Byte 3 determines left (0x00) vs right (0x01) screen
🎨 **RGB565 Colors**: Standard 16-bit color format works perfectly
📐 **Simple Coordinates**: Basic X,Y positioning system
⚡ **32-Byte Packets**: Much simpler than expected - no complex RLE needed
🔚 **Consistent Terminator**: Always ends with 0x40 command + display ID

## Troubleshooting

If the test doesn't work:

1. **Check USB Interface**: Make sure Interface 5 is accessible
2. **Driver Issues**: May need WinUSB driver (use Zadig tool)
3. **Interface Claiming**: The device needs proper USB interface access
4. **Endpoint Access**: Bulk endpoint 0x04 must be writable

## Next Steps

If this test succeeds, you can:
- Build more complex graphics using multiple 32-byte packets
- Implement full screen updates by sending many small rectangles
- Create animation by rapidly updating different screen regions
- Test the larger packet formats for full-screen bitmap updates

# MK3 Display Protocol - Next Steps

## Current Status ✅
- **Basic communication works** - Device responds to USB bulk transfers
- **Color channels identified** - R→B, G→R, B→G rotation required  
- **32-byte packets work** - Can draw lines/rectangles (position issues remain)
- **Display ID confirmed** - 0x00=left, 0x01=right screen

## Next Phase: Full Screen Protocol 🎯

### Objective
Parse and replicate the **full screen initialization** packets from Wireshark captures to achieve proper screen fills.

### Strategy
1. **Parse init captures** - Extract the large (2KB+) packets that fill entire screens
2. **Send exact replicas** - Verify our USB communication can handle large packets  
3. **Decode format** - Understand the full-screen bitmap/RLE protocol
4. **Implement properly** - Create working full-screen graphics

### Available Captures for Analysis
- `blank init 1.pcapng` - Device startup sequence
- `blank init 2.pcapng` - Second init capture
- `blank init 3.pcapng` - Third init capture  
- `blank_to_midimode.pcapng` - Full screen change (both displays)
- `holdshift_screenswap.pcapng` - Complete screen swap
- `switchpage.pcapng` - Page change (likely full screen)

### Step 1: Extract Init Packet
**Goal**: Find the first large packet that fills a screen with solid content.

**Method**:
```bash
tshark -r "blank init 1.pcapng" -Y "usb.endpoint_address.number == 4 && frame.len > 1000" -V -x
```

Look for:
- Large packets (>1000 bytes)  
- Bulk transfers to endpoint 0x04
- Packets that would fill entire 480x272 screen

### Step 2: Create Exact Replica Test
**Goal**: Send the exact same bytes to verify our USB stack works with large packets.

**Implementation**:
```rust
// Extract packet bytes from tshark output
let init_packet = vec![
    // Paste exact hex bytes from capture here
    0x84, 0x00, 0x00, 0x60, /* ... rest of packet ... */
];

device.write_display(&init_packet)?;
```

### Step 3: Decode Protocol Structure  
**Goal**: Understand the format of large screen-fill packets.

**Analysis points**:
- Header structure (first 32 bytes)
- Bitmap vs RLE encoding
- Color format in large packets
- Compression/encoding method

### Step 4: Test Full Screen Implementation
**Goal**: Generate our own full-screen packets using discovered format.

### Expected Outcome
- ✅ **Working full-screen fills** with solid colors
- ✅ **Complete protocol documentation** 
- ✅ **Foundation for complex graphics** (bitmaps, patterns, etc.)

## Current Code Status
- **Working examples**: `protocol_test_simple.rs`, `protocol_test_correct_colors.rs`
- **Color correction function**: `rgb565_corrected()` 
- **USB communication**: `device.write_display()` confirmed working
- **Analysis docs**: `docs/packet_analysis.md`

## Handoff Notes
1. **Focus on large packets** - Small packet coordinate issues can be solved later
2. **Exact replication first** - Verify we can send large packets before decoding
3. **Use working captures** - We have multiple full-screen change examples
4. **Build incrementally** - Start with one exact replica, then understand format

The foundation is solid - now we need to crack the full-screen protocol! 🚀

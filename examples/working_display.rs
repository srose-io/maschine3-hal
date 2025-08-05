use std::io::Result;
use mk3_hal::MaschineMK3;

const EACH_WIDTH: u16 = 480;
const EACH_HEIGHT: u16 = 272; 
const PIXEL_LENGTH: usize = 2; // RGB565 = 2 bytes per pixel
const HEADER_LENGTH: usize = 16;
const COMMAND_LENGTH: usize = 4;

fn rgb565(red: u8, green: u8, blue: u8) -> u16 {
    ((red & 0x1f) as u16) << 11 | ((green & 0x3f) as u16) << 5 | (blue & 0x1f) as u16
}

fn fill_header(buf: &mut [u8], display_num: u8, x: u16, y: u16, width: u16, height: u16) {
    // Based on working JavaScript implementation
    buf[0] = 0x84;   // Command type
    buf[1] = 0x00;   // Always 0
    buf[2] = display_num; // Display ID (0=left, 1=right)
    buf[3] = 0x60;   // Always 0x60
    // bytes 4-7 are 0x00
    
    // Coordinates at offsets 8-15 (corrected based on working code)
    buf[8] = (x >> 8) as u8;     // X MSB
    buf[9] = (x & 0xff) as u8;   // X LSB
    buf[10] = (y >> 8) as u8;    // Y MSB  
    buf[11] = (y & 0xff) as u8;  // Y LSB
    buf[12] = (width >> 8) as u8;  // Width MSB
    buf[13] = (width & 0xff) as u8; // Width LSB
    buf[14] = (height >> 8) as u8;  // Height MSB
    buf[15] = (height & 0xff) as u8; // Height LSB
}

fn fill_transmit_command(buf: &mut [u8], num_pixels: u32, offset: usize) {
    let half_pixels = num_pixels / 2;
    
    // Command 0x00 = transmit pixels
    buf[offset] = 0x00;
    buf[offset + 1] = (half_pixels >> 16) as u8; // MSB  
    buf[offset + 2] = (half_pixels >> 8) as u8;  // Middle
    buf[offset + 3] = (half_pixels & 0xff) as u8; // LSB
}

fn make_gradient_buffer(width: u16, height: u16) -> Vec<u8> {
    let num_pixels = (width as usize) * (height as usize);
    let mut buffer = vec![0u8; num_pixels * PIXEL_LENGTH];
    
    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y as usize * width as usize + x as usize) * 2;
            
            // Create a red-to-blue gradient across X, green gradient across Y
            let red = ((x as f32 / width as f32) * 31.0) as u8;
            let green = ((y as f32 / height as f32) * 63.0) as u8;
            let blue = (((width - x) as f32 / width as f32) * 31.0) as u8;
            
            let rgb = rgb565(red, green, blue);
            
            // Store as little-endian
            buffer[pixel_idx] = (rgb & 0xff) as u8;     // LSB
            buffer[pixel_idx + 1] = (rgb >> 8) as u8;   // MSB
        }
    }
    
    buffer
}

fn make_checkerboard_buffer(width: u16, height: u16, square_size: u16) -> Vec<u8> {
    let num_pixels = (width as usize) * (height as usize);
    let mut buffer = vec![0u8; num_pixels * PIXEL_LENGTH];
    
    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y as usize * width as usize + x as usize) * 2;
            
            let checker_x = (x / square_size) % 2;
            let checker_y = (y / square_size) % 2;
            
            let rgb = if (checker_x + checker_y) % 2 == 0 {
                rgb565(31, 63, 31) // Bright green
            } else {
                rgb565(31, 0, 31)  // Magenta  
            };
            
            // Store as little-endian
            buffer[pixel_idx] = (rgb & 0xff) as u8;     // LSB
            buffer[pixel_idx + 1] = (rgb >> 8) as u8;   // MSB
        }
    }
    
    buffer
}

fn paint_display_row_by_row(device: &MaschineMK3, display_num: u8, pixel_buffer: &[u8]) -> Result<()> {
    let width = EACH_WIDTH;
    let height = 1; // Paint one row at a time like the working implementation
    let num_pixels = width as u32;
    
    println!("🎨 Painting display {} row by row...", display_num);
    
    for row in 0..EACH_HEIGHT {
        // Create packet for this row
        let packet_size = HEADER_LENGTH + COMMAND_LENGTH * 3 + (num_pixels as usize * PIXEL_LENGTH);
        let mut packet = vec![0u8; packet_size];
        
        // Fill header for this row
        fill_header(&mut packet, display_num, 0, row, width, height);
        
        // Fill transmit command
        fill_transmit_command(&mut packet, num_pixels, HEADER_LENGTH);
        
        // Copy pixel data for this row
        let data_start = HEADER_LENGTH + COMMAND_LENGTH;
        let row_start = row as usize * width as usize * PIXEL_LENGTH;
        let row_end = row_start + (width as usize * PIXEL_LENGTH);
        
        if row_end <= pixel_buffer.len() {
            packet[data_start..data_start + (width as usize * PIXEL_LENGTH)]
                .copy_from_slice(&pixel_buffer[row_start..row_end]);
        }
        
        // Add blit command (0x03)
        let blit_offset = data_start + (width as usize * PIXEL_LENGTH);
        packet[blit_offset] = 0x03;
        packet[blit_offset + 1] = 0x00;
        packet[blit_offset + 2] = 0x00;
        packet[blit_offset + 3] = 0x00;
        
        // Add end command (0x40)
        let end_offset = blit_offset + COMMAND_LENGTH;
        packet[end_offset] = 0x40;
        packet[end_offset + 1] = 0x00;
        packet[end_offset + 2] = 0x00;
        packet[end_offset + 3] = 0x00;
        
        // Send packet
        if let Err(e) = device.send_raw_data(&packet) {
            eprintln!("Failed to send row {}: {}", row, e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Row {} failed: {}", row, e)));
        }
        
        // Small delay between rows to avoid overwhelming the device
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    
    println!("✅ Display {} painted successfully", display_num);
    Ok(())
}

fn main() -> Result<()> {
    println!("🎨 Testing working display implementation based on JavaScript code...");
    
    // Initialize device
    let device = match MaschineMK3::new() {
        Ok(dev) => dev,
        Err(e) => {
            eprintln!("Failed to initialize MK3 device: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found"));
        }
    };
    
    println!("Device initialized successfully");
    
    // Create gradient pattern for display 0  
    println!("\n🌈 Creating gradient pattern for left display...");
    let gradient_pixels = make_gradient_buffer(EACH_WIDTH, EACH_HEIGHT);
    
    if let Err(e) = paint_display_row_by_row(&device, 0, &gradient_pixels) {
        eprintln!("Failed to paint gradient on display 0: {}", e);
    }
    
    std::thread::sleep(std::time::Duration::from_millis(2000));
    
    // Create checkerboard pattern for display 1
    println!("\n♟️ Creating checkerboard pattern for right display...");
    let checker_pixels = make_checkerboard_buffer(EACH_WIDTH, EACH_HEIGHT, 32);
    
    if let Err(e) = paint_display_row_by_row(&device, 1, &checker_pixels) {
        eprintln!("Failed to paint checkerboard on display 1: {}", e);
    }
    
    println!("\n🎉 Working display test complete!");
    Ok(())
}

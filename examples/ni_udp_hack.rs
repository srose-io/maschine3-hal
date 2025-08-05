use std::net::UdpSocket;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Native Instruments UDP Protocol Hacking");
    println!("===========================================");
    println!("Target: 127.0.0.1:7579 (NIHostIntegrationAgent)");
    println!();

    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;
    socket.set_write_timeout(Some(Duration::from_millis(500)))?;
    
    let ni_service = "127.0.0.1:7579";
    
    // Test different message patterns based on Rebellion research
    let test_messages = vec![
        // Basic probes
        ("HELLO", "Basic greeting"),
        ("PING", "Simple ping"),
        ("PROBE", "Service probe"),
        
        // NI-specific patterns (from Rebellion project analysis)
        ("NI_DISCOVER", "NI discovery message"),
        ("NIHA_HELLO", "NIHA greeting"),
        ("NIHA_CONNECT", "NIHA connection request"),
        
        // JSON-like messages
        ("{\"type\":\"discover\"}", "JSON discovery"),
        ("{\"cmd\":\"connect\"}", "JSON connect"),
        
        // Rebellion-style messages (based on code analysis)
        ("NIIPC_CONNECT", "NIIPC connection"),
        ("REBELLION_HELLO", "Rebellion greeting"),
    ];
    
    for (message, description) in test_messages {
        println!("📤 Testing: {} ({} bytes)", description, message.len());
        
        match socket.send_to(message.as_bytes(), ni_service) {
            Ok(bytes_sent) => {
                println!("   ✅ Sent {} bytes", bytes_sent);
                
                // Try to receive response
                let mut buffer = [0u8; 1024];
                match socket.recv_from(&mut buffer) {
                    Ok((bytes_received, from_addr)) => {
                        println!("   🎉 RESPONSE! {} bytes from {}", bytes_received, from_addr);
                        println!("   📄 Data (hex): {:02x?}", &buffer[..bytes_received.min(32)]);
                        println!("   📄 Data (string): {:?}", 
                                String::from_utf8_lossy(&buffer[..bytes_received.min(64)]));
                        println!();
                    }
                    Err(_) => {
                        println!("   ⚠️  No response (timeout)");
                    }
                }
            }
            Err(e) => {
                println!("   ❌ Send failed: {}", e);
            }
        }
        
        // Small delay between tests
        std::thread::sleep(Duration::from_millis(100));
    }
    
    println!("🔬 Advanced Protocol Testing");
    println!("============================");
    
    // Test some more sophisticated patterns
    
    // 1. Try a Maschine MK3 device identification
    println!("📤 Testing Maschine MK3 device identification...");
    let mk3_id = [0x17, 0xCC, 0x16, 0x00]; // VID:0x17CC PID:0x1600 in little endian
    match socket.send_to(&mk3_id, ni_service) {
        Ok(_) => {
            let mut buffer = [0u8; 1024];
            if let Ok((bytes, addr)) = socket.recv_from(&mut buffer) {
                println!("   🎉 Device ID response: {} bytes from {}", bytes, addr);
                println!("   📄 Data: {:02x?}", &buffer[..bytes.min(32)]);
            } else {
                println!("   ⚠️  No device ID response");
            }
        }
        Err(e) => println!("   ❌ Device ID send failed: {}", e),
    }
    
    // 2. Try display protocol header
    println!("📤 Testing display protocol header...");
    let display_header = [0x84, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xe0, 0x01, 0x10];
    match socket.send_to(&display_header, ni_service) {
        Ok(_) => {
            let mut buffer = [0u8; 1024];
            if let Ok((bytes, addr)) = socket.recv_from(&mut buffer) {
                println!("   🎉 Display protocol response: {} bytes from {}", bytes, addr);
                println!("   📄 Data: {:02x?}", &buffer[..bytes.min(32)]);
            } else {
                println!("   ⚠️  No display protocol response");
            }
        }
        Err(e) => println!("   ❌ Display protocol send failed: {}", e),
    }
    
    println!();
    println!("🏁 Protocol hacking complete!");
    println!("💡 If we got responses, we've found the communication protocol!");
    println!("   Next step: Reverse engineer the message format and implement proper client");
    
    Ok(())
}

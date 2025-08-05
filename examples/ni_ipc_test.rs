use mk3_hal::ni_ipc::{discover_ni_services, discover_all_ni_services, NiIpcClient};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Native Instruments IPC Discovery Test");
    println!("⚠️  Make sure NI services are running (run as admin: net start NIHardwareService)");
    println!();
    
    // Run comprehensive discovery first
    discover_all_ni_services()?;
    println!();

    // Step 1: Try to discover available pipes
    println!("Step 1: Checking for available NI named pipes...");
    match NiIpcClient::list_pipes() {
        Ok(pipes) => {
            if pipes.is_empty() {
                println!("❌ No NI pipes found. Make sure NI services are running.");
            } else {
                println!("✅ Found {} potential NI pipes:", pipes.len());
                for pipe in &pipes {
                    println!("   - {}", pipe);
                }
            }
        }
        Err(e) => {
            println!("❌ Error listing pipes: {}", e);
        }
    }
    println!();

    // Step 2: Try to connect to discovered services
    println!("Step 2: Attempting to connect to NI services...");
    match discover_ni_services() {
        Ok(mut clients) => {
            if clients.is_empty() {
                println!("❌ Could not connect to any NI services");
                println!("   Try starting NI services manually:");
                println!("   - Run as admin: net start NIHardwareService");
                println!("   - Run as admin: net start NIHostIntegrationAgent");
            } else {
                println!("✅ Successfully connected to {} NI services!", clients.len());
                
                // Step 3: Try basic communication with each service
                for (i, client) in clients.iter().enumerate() {
                    println!();
                    println!("Step 3.{}: Testing communication with service {}...", i + 1, i + 1);
                    
                    // Try sending a simple probe message
                    let probe_message = b"PROBE";
                    match client.send(probe_message) {
                        Ok(()) => {
                            println!("   ✅ Successfully sent probe message");
                            
                            // Try to receive a response
                            let mut response_buffer = [0u8; 1024];
                            match client.receive(&mut response_buffer) {
                                Ok(bytes_received) => {
                                    println!("   ✅ Received {} bytes response", bytes_received);
                                    if bytes_received > 0 {
                                        println!("   📄 Response data (hex): {:02x?}", 
                                                &response_buffer[..bytes_received.min(32)]);
                                    }
                                }
                                Err(e) => {
                                    println!("   ⚠️  No response received: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("   ❌ Failed to send probe message: {}", e);
                        }
                    }
                    
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
        Err(e) => {
            println!("❌ Error discovering NI services: {}", e);
        }
    }
    println!();

    // Step 4: Manual pipe testing
    println!("Step 4: Manual testing of specific pipe names...");
    let test_pipes = vec![
        "NIHardwareService",
        "NIHostIntegrationAgent",
        "NIHA",
        "NIHIA", 
        "NativeInstruments",
        "Maschine2",
        "KompleteKontrol",
        "Maschine",
        "Kontakt",
    ];

    for pipe_name in test_pipes {
        print!("   Testing pipe '{}': ", pipe_name);
        let mut client = NiIpcClient::new(pipe_name);
        match client.connect() {
            Ok(()) => {
                println!("✅ Connected!");
                
                // Try to send a simple message
                if let Ok(()) = client.send(b"HELLO") {
                    println!("     📤 Sent greeting");
                    
                    // Try to get a response with timeout
                    let mut buffer = [0u8; 256];
                    match client.receive(&mut buffer) {
                        Ok(bytes) if bytes > 0 => {
                            println!("     📥 Got {} bytes response: {:02x?}", 
                                    bytes, &buffer[..bytes.min(16)]);
                        }
                        _ => {
                            println!("     ⚠️  No response");
                        }
                    }
                }
            }
            Err(_) => {
                println!("❌ Failed to connect");
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    println!();
    println!("🏁 NI IPC Discovery Test Complete!");
    println!("💡 If no pipes were found, try:");
    println!("   1. Run as administrator");
    println!("   2. Start NI services: net start NIHardwareService");
    println!("   3. Launch Maschine 2 or Komplete Kontrol software");

    Ok(())
}

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, BOOL};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE,
    OPEN_EXISTING,
};
use windows::Win32::System::Pipes::WaitNamedPipeW;

use crate::error::{MK3Error, Result};

/// Native Instruments IPC client for communicating with NI services via named pipes
pub struct NiIpcClient {
    pipe_handle: HANDLE,
    pipe_name: String,
}

impl NiIpcClient {
    /// Create a new NI IPC client for the specified pipe name
    pub fn new(pipe_name: &str) -> Self {
        Self {
            pipe_handle: INVALID_HANDLE_VALUE,
            pipe_name: format!(r"\\.\pipe\{}", pipe_name),
        }
    }

    /// List available named pipes (for discovery)
    pub fn list_pipes() -> Result<Vec<String>> {
        // On Windows, we can't easily enumerate all pipes, but we can try common NI pipe names
        // Based on Rebellion project and reverse engineering attempts
        let common_ni_pipes = vec![
            "NIHardwareService",
            "NIHostIntegrationAgent", 
            "NIHA",
            "NIHIA",
            "Maschine2",
            "KompleteKontrol",
            "NativeInstruments",
            // Additional patterns found in research
            "NI_Hardware",
            "NI_Host",
            "NI_Communication",
            "NI_IPC",
            "ni_hardware_service", 
            "ni_host_integration",
            // Process ID based patterns (services might append PID)
            // (Note: these need to be handled separately due to lifetime issues)
            // Different naming conventions
            "ni.hardware.service",
            "ni.host.integration",
            "native-instruments",
        ];

        let mut available_pipes = Vec::new();
        
        for pipe_name in common_ni_pipes {
            let full_name = format!(r"\\.\pipe\{}", pipe_name);
            if Self::pipe_exists(&full_name) {
                available_pipes.push(pipe_name.to_string());
            }
        }

        // Add dynamic pipe names with current process ID
        let pid_pipes = vec![
            format!("NIHardwareService_{}", std::process::id()),
            format!("NIHA_{}", std::process::id()),
        ];
        
        for pipe_name in pid_pipes {
            let full_name = format!(r"\\.\pipe\{}", pipe_name);
            if Self::pipe_exists(&full_name) {
                available_pipes.push(pipe_name);
            }
        }

        Ok(available_pipes)
    }

    /// Check if a named pipe exists
    fn pipe_exists(pipe_name: &str) -> bool {
        let wide_name = Self::to_wide_string(pipe_name);
        unsafe {
            let result: BOOL = WaitNamedPipeW(PCWSTR(wide_name.as_ptr()), 0);
            result.as_bool()
        }
    }

    /// Convert string to wide string for Windows APIs
    fn to_wide_string(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
    }

    /// Connect to the named pipe
    pub fn connect(&mut self) -> Result<()> {
        println!("🔌 Attempting to connect to pipe: {}", self.pipe_name);

        let wide_name = Self::to_wide_string(&self.pipe_name);

        // Wait for the pipe to become available (up to 5 seconds)
        unsafe {
            let result: BOOL = WaitNamedPipeW(PCWSTR(wide_name.as_ptr()), 5000);
            if !result.as_bool() {
                return Err(MK3Error::InvalidPacket); // Use available error type
            }
        }

        // Open the pipe with read/write access
        unsafe {
            match CreateFileW(
                PCWSTR(wide_name.as_ptr()),
                0x80000000 | 0x40000000, // GENERIC_READ | GENERIC_WRITE
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            ) {
                Ok(handle) => self.pipe_handle = handle,
                Err(_) => return Err(MK3Error::InvalidPacket),
            }
        }

        if self.pipe_handle == INVALID_HANDLE_VALUE {
            return Err(MK3Error::InvalidPacket);
        }

        println!("✅ Connected to pipe: {}", self.pipe_name);
        Ok(())
    }

    /// Send data to the pipe
    pub fn send(&self, data: &[u8]) -> Result<()> {
        if self.pipe_handle == INVALID_HANDLE_VALUE {
            return Err(MK3Error::InvalidPacket);
        }

        let mut bytes_written = 0u32;
        unsafe {
            match WriteFile(
                self.pipe_handle,
                Some(data),
                Some(&mut bytes_written),
                None,
            ) {
                Ok(_) => {},
                Err(_) => return Err(MK3Error::InvalidPacket),
            }
        }

        println!("📤 Sent {} bytes to pipe", bytes_written);
        Ok(())
    }

    /// Receive data from the pipe
    pub fn receive(&self, buffer: &mut [u8]) -> Result<usize> {
        if self.pipe_handle == INVALID_HANDLE_VALUE {
            return Err(MK3Error::InvalidPacket);
        }

        let mut bytes_read = 0u32;
        unsafe {
            match ReadFile(
                self.pipe_handle,
                Some(buffer),
                Some(&mut bytes_read),
                None,
            ) {
                Ok(_) => {},
                Err(_) => return Err(MK3Error::InvalidPacket),
            }
        }

        println!("📥 Received {} bytes from pipe", bytes_read);
        Ok(bytes_read as usize)
    }

    /// Send a message and wait for response
    pub fn send_and_receive(&self, request: &[u8], response_buffer: &mut [u8]) -> Result<usize> {
        self.send(request)?;
        self.receive(response_buffer)
    }

    /// Disconnect from the pipe
    pub fn disconnect(&mut self) {
        if self.pipe_handle != INVALID_HANDLE_VALUE {
            unsafe {
                let _ = CloseHandle(self.pipe_handle);
            }
            self.pipe_handle = INVALID_HANDLE_VALUE;
            println!("🔌 Disconnected from pipe: {}", self.pipe_name);
        }
    }
}

impl Drop for NiIpcClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// Try to discover and connect to NI services
pub fn discover_ni_services() -> Result<Vec<NiIpcClient>> {
    println!("🔍 Discovering NI services via named pipes...");
    
    let available_pipes = NiIpcClient::list_pipes()?;
    println!("📋 Found {} potential NI pipes: {:?}", available_pipes.len(), available_pipes);

    let mut connected_clients = Vec::new();

    for pipe_name in available_pipes {
        let mut client = NiIpcClient::new(&pipe_name);
        match client.connect() {
            Ok(()) => {
                println!("✅ Successfully connected to: {}", pipe_name);
                connected_clients.push(client);
            }
            Err(e) => {
                println!("❌ Failed to connect to {}: {}", pipe_name, e);
            }
        }
    }

    Ok(connected_clients)
}

/// Try to discover NI services via UDP (based on Rebellion project findings)
pub fn discover_ni_udp_services() -> Result<Vec<SocketAddr>> {
    println!("🌐 Discovering NI services via UDP...");
    
    // Common ports that NI services might use (based on research and discovery)
    let test_ports = vec![
        7579, // DISCOVERED: NIHostIntegrationAgent actual port!
        7001, 7002, 7003, 7004, 7005, // Common NI range
        3456, 3457, 3458, // Alternative range
        8080, 8081, 8082, // Web service range
        2049, 2050, 2051, // Network range
        5000, 5001, 5002, // Common service range
    ];
    
    let mut discovered_services = Vec::new();
    
    for port in test_ports {
        let addr = format!("127.0.0.1:{}", port);
        if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
            match test_udp_service(socket_addr) {
                Ok(true) => {
                    println!("✅ Found responsive UDP service at {}", socket_addr);
                    discovered_services.push(socket_addr);
                }
                Ok(false) => {
                    // Service not responsive, but that's normal
                }
                Err(_) => {
                    // Connection failed, also normal
                }
            }
        }
    }
    
    println!("📋 Found {} responsive UDP services", discovered_services.len());
    Ok(discovered_services)
}

/// Test if a UDP service is responsive at the given address
fn test_udp_service(addr: SocketAddr) -> Result<bool> {
    match UdpSocket::bind("127.0.0.1:0") {
        Ok(socket) => {
            socket.set_read_timeout(Some(Duration::from_millis(100)))?;
            socket.set_write_timeout(Some(Duration::from_millis(100)))?;
            
            // Send a probe message (similar to what Rebellion might use)
            let probe_msg = b"PROBE_NI_SERVICE";
            match socket.send_to(probe_msg, addr) {
                Ok(_) => {
                    // Try to receive a response
                    let mut buffer = [0u8; 1024];
                    match socket.recv_from(&mut buffer) {
                        Ok((bytes_received, _)) => {
                            println!("   📥 Received {} bytes from {}", bytes_received, addr);
                            Ok(true)
                        }
                        Err(_) => Ok(false), // No response (timeout)
                    }
                }
                Err(_) => Ok(false), // Send failed
            }
        }
        Err(_) => Err(MK3Error::InvalidPacket),
    }
}

/// Comprehensive NI service discovery (both pipes and UDP)
pub fn discover_all_ni_services() -> Result<()> {
    println!("🔍 Comprehensive NI Service Discovery");
    println!("=====================================\n");
    
    // Test named pipes
    println!("1️⃣  Testing Named Pipes:");
    match discover_ni_services() {
        Ok(clients) => {
            if clients.is_empty() {
                println!("   ❌ No named pipe services found");
            } else {
                println!("   ✅ Found {} named pipe services", clients.len());
            }
        }
        Err(e) => println!("   ❌ Named pipe discovery error: {}", e),
    }
    
    println!();
    
    // Test UDP services  
    println!("2️⃣  Testing UDP Services:");
    match discover_ni_udp_services() {
        Ok(services) => {
            if services.is_empty() {
                println!("   ❌ No UDP services found");
            } else {
                println!("   ✅ Found {} UDP services", services.len());
                for service in services {
                    println!("      - {}", service);
                }
            }
        }
        Err(e) => println!("   ❌ UDP discovery error: {}", e),
    }
    
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_discovery() {
        match NiIpcClient::list_pipes() {
            Ok(pipes) => println!("Available pipes: {:?}", pipes),
            Err(e) => println!("Error discovering pipes: {}", e),
        }
    }

    #[test]
    fn test_ni_service_discovery() {
        match discover_ni_services() {
            Ok(clients) => println!("Connected to {} NI services", clients.len()),
            Err(e) => println!("Error connecting to NI services: {}", e),
        }
    }
}

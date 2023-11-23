use std::io::BufRead;
use std::net;
use std::io;
use std::process::Command;
use regex::Regex;

pub struct ScanFunc;
impl ScanFunc {
    
    const COMMON_PORTS: [u16; 40] = [
        80,    // HTTP
        443,   // HTTPS
        21,    // FTP
        22,    // SSH
        25,    // SMTP
        110,   // POP3
        143,   // IMAP
        53,    // DNS
        3389,  // RDP
        137,   // NetBIOS
        138,   // NetBIOS
        139,   // NetBIOS
        445,   // SMB
        3306,  // MySQL
        5432,  // PostgreSQL
        8080,  // HTTP Alternate
        23,    // Telnet
        179,   // BGP (Border Gateway Protocol)
        465,   // SMTPS (SMTP Secure)
        587,   // SMTP (Message Submission)
        636,   // LDAPS (LDAP Secure)
        993,   // IMAPS (IMAP Secure)
        995,   // POP3S (POP3 Secure)
        1723,  // PPTP (Point-to-Point Tunneling Protocol)
        2049,  // NFS (Network File System)
        3268,  // Global Catalog (LDAP)
        3269,  // Global Catalog (LDAP Secure)
        5433,  // Redis
        5985,  // WinRM (Windows Remote Management)
        5986,  // WinRM (Windows Remote Management Secure)
        8081,  // HTTP Alternate
        8443,  // HTTPS Alternate
        9000,  // SonarQube
        9090,  // Openfire
        9091,  // Openfire Secure
        9100,  // Printer (JetDirect)
        9200,  // Elasticsearch REST API
        9300,  // Elasticsearch Transport
        9418,  // Git (Gitsync)
        27017, // MongoDB
    ];
    

    pub fn connect(ip: &str, port: u16) -> bool {
        // Attempt to connect to the server.
        let socket = match net::TcpStream::connect((ip, port)) {
            Ok(socket) => socket,
            Err(_) => {
                println!("Failed to connect to {}:{}", ip, port);
                return false;
            }
        };
        // Get the response from the server.
        let response = ScanFunc::get_server_response(socket);
        println!("Response: {}", response);
        return true;
    }

    pub fn basic_scan_params() -> (String, u16, u16) {
        println!("Enter IP address to scan:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let ip_address = input.trim().to_string(); 
    
        println!("Enter minimum port to scan:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let min_port = input.trim().parse::<u16>().expect("Invalid port number");
    
        println!("Enter maximum port to scan:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let max_port = input.trim().parse::<u16>().expect("Invalid port number");
    
        println!("Scanning {} from port {} to port {}", ip_address, min_port, max_port);
    
        return (ip_address, min_port, max_port);
    }
    
    pub fn basic_scan(ip_address: &str, min_port: u16, max_port: u16) {
        for port in min_port..max_port {
            if ScanFunc::connect(ip_address, port) {
                println!("Port {} is open", port);
            }
        }
    }

    pub fn basic_scan_banner(ip_address: &str, min_port: u16, max_port: u16) {
        for port in min_port..max_port {
            let socket = match net::TcpStream::connect((ip_address, port)) {
                Ok(socket) => socket,
                Err(_) => {
                    println!("Failed to connect to {}:{}", ip_address, port);
                    continue;
                }
            };
            let response = ScanFunc::get_server_response(socket);
            let banner = ScanFunc::get_banner(&response);
            println!("Port {} is open: {}", port, banner);
        }
    }

    fn get_server_response(socket: net::TcpStream) -> String {
        let mut reader = io::BufReader::new(socket);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        String::from_utf8(buffer).expect("Could not write buffer as string")
    }

    fn get_banner(response: &str) -> &str {
        let lines: Vec<&str> = response.lines().collect();

        // Check if it's an HTTP response (e.g., "HTTP/1.1 200 OK").
        if lines.len() > 0 && lines[0].to_lowercase().starts_with("http/") {
            // Extract the status line (e.g., "HTTP/1.1 200 OK").
            let status_line = lines[0];
            
            // Return the extracted status line as the banner.
            return status_line;
        } else {
            // If it's not an HTTP response, you can implement similar logic for other services.
            // For now, just return the entire response as the banner.
            return response.trim();
        }
    }

    pub fn simple_scan_most_common_ports() {
        println!("Enter IP address to scan:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let ip_address = input.trim().to_string();

        for port in Self::COMMON_PORTS.iter() {
            if ScanFunc::connect(&ip_address, *port) {
                println!("Port {} is open", port);
            }
            else{
                println!("Port {} is closed", port);
            }
        }
    }

    pub fn ping_for_ip() {
        println!("Enter IPv4 address to ping:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let ip_address = input.trim().to_string();
    
        let output = Command::new("ping")
            .arg("-4")  // Force IPv4
            .arg("-c")
            .arg("4")   // Number of pings
            .arg(ip_address)
            .output()
            .expect("Failed to execute command");
    

            // Convert the output to a string
            let output_string = String::from_utf8_lossy(&output.stdout);

            // Use a regular expression to extract the IP address
            let ip_regex = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").expect("Invalid regex");
            if let Some(captured_ip) = ip_regex.find(&output_string) {
                println!("Extracted IP address: {}", captured_ip.as_str());
            } else {
                println!("No IP address found in the ping output");
            }
    }

}

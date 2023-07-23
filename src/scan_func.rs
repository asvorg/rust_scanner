use std::io::BufRead;
use std::net;
use std::io;

pub struct ScanFunc;
impl ScanFunc {

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

    fn get_server_response(socket: net::TcpStream) -> String {
        let mut reader = io::BufReader::new(socket);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        String::from_utf8(buffer).expect("Could not write buffer as string")
    }
    
}    
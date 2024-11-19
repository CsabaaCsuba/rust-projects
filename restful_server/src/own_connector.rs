use std::net::TcpStream;
use std::io::{Write, Read, Error};

pub fn test_raw_conn() -> Result<(), Error> {
    let mysql_host = "localhost";  // MySQL server address
    let mysql_port = 3306;         // MySQL default port
    let mysql_user = "root";       // MySQL username
    let mysql_password = "";      // MySQL password (empty by default on XAMPP)

    // Step 1: Connect to MySQL server
    let mut stream = TcpStream::connect((mysql_host, mysql_port))?;
    println!("Connected to MySQL server at {}:{}", mysql_host, mysql_port);

    // Step 2: Send a simplified authentication and query packet
    // For simplicity, we're skipping the handshake and authentication steps here.

    // Construct a simple query packet (we're sending "SELECT 1")
    // This is just a very simplified representation. Real packets require specific headers and fields.

    let query = "SELECT * from mylittledb";
    let packet_length = query.len() as u32 + 1; // Simplified length calculation
    let mut packet = Vec::with_capacity(packet_length as usize);

    packet.push(0x03);  // Packet type (0x03 is for a query command in MySQL protocol)
    packet.push(0x00);  // Sequence ID (simplified, should be incremented with each packet)
    packet.push(0x00);  // Command byte (0x03 means query)
    
    // Add the actual SQL query
    packet.extend_from_slice(query.as_bytes());

    // Step 3: Send the packet to MySQL
    stream.write_all(&packet)?;

    // Step 4: Read the response from MySQL (this will just be raw bytes in this case)
    let mut response = vec![0; 1024];  // Buffer to store MySQL response
    let n = stream.read(&mut response)?;

    // Step 5: Print out the raw response for now
    let lossy_string = String::from_utf8_lossy(&response[..n]);

    // Step 6: Print out the lossy string
    println!("Received response (lossy string): {}", lossy_string);

    Ok(())
}
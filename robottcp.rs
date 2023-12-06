use std::io::{self, Read, Write};
use std::net::{TcpStream, SocketAddr};

fn main() -> io::Result<()> {
    // Skapa en socket
    let client_socket = match TcpStream::connect("11234444") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return Err(e);
        }
    };
       
    println!("Connected to server");

    // Skicka och ta emot data här
    let message_to_send = "Hello, server!";
    client_socket.write_all(message_to_send.as_bytes())?;

    let mut buffer = [0u8; 4000];
    let bytes_received = client_socket.read(&mut buffer)?;

    if bytes_received > 0 {
        let received_message = String::from_utf8_lossy(&buffer[..bytes_received]);
        println!("Received from server: {}", received_message);
    } else {
        println!("No data received from server");
    }

    // Stäng socket (automatically closed when client_socket goes out of scope)

    Ok(())
}

use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;

/**
 * Read and decode ADSB message in SBS1 format using library from https://github.com/samcrow/rust-sbs1
 */
pub fn decode(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    
    loop {
        let mut buf = String::new();

        // Check and read one line from TCP stream
        match reader.read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    println!("Disconnected");
                    break;
                } else {
                    println!("Got {} bytes", n);
                    println!("{}", buf.trim());

                    // Use decoder from https://github.com/samcrow/rust-sbs1
                    let message = sbs1::parse(&buf).unwrap();
                    println!("{:?}", message.message_type);
                
                }    
            }
            Err(e) => {
                println!("Error reading: {}", e);
                break;
            }
        }
    }
}
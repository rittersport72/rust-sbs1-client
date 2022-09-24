pub mod decoder;

use std::env;
use std::net::TcpStream;

/**
 * Application entry point.
 * 
 * Connect as TCP client to running dump1090.
 * Read command line arguments for IP address and port number.
 * Otherwise use default 127.0.0.1 and 30003.
 */
fn main() {
    // Default ip address and port number
    let mut ip = "127.0.0.1";
    let mut port = "30003";
    
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() == 3 {
        ip = &args[1];
        port = &args[2];
    }

    let address = ip.to_string() + ":" + &port.to_string();
    
    match TcpStream::connect(&address) {
        Ok(stream) => {
            println!("Connected");
            
            decoder::decode(stream);
        }
        Err(err) => {
            println!("Error {}", err);
        }
    }
    println!("Disconnected");
}

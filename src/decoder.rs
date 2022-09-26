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
                    match sbs1::parse(&buf) {
                        Ok(message) => {
                            //println!("{:?}", message.message_type);
                            print_message(&message);
                        }
                        Err(e) => {
                            println!("Error parsing: {}", e.to_string());
                        }
                    }
                    
                }    
            }
            Err(e) => {
                println!("Error reading: {}", e);
                break;
            }
        }
    }
}

/**
* Print some attributes of decoded SBS1 message.
*/
fn print_message(message: &sbs1::Message) {
    //println!("{:?}", message);
    
    match message.message_type {
        sbs1::MessageType::Transmission(sbs1::TransmissionType::EsIdentAndCategory) => {
            println!("{} callsign {}", message.ident.unwrap(), message.callsign.as_ref().unwrap());
        }
        sbs1::MessageType::Transmission(sbs1::TransmissionType::EsAirbornePos) => {
            println!("{} latitude {} longitude {}", message.ident.unwrap(), message.latitude.unwrap(), message.longitude.unwrap());
        }
        sbs1::MessageType::Transmission(sbs1::TransmissionType::EsAirborneVel) => {
            println!("{} groundspeed {}", message.ident.unwrap(), message.ground_speed.unwrap());
        }
        sbs1::MessageType::Transmission(sbs1::TransmissionType::SurveillanceAlt) => {
            println!("{} altitude {}", message.ident.unwrap(), message.altitude.unwrap());
        }
        _ => { 
            println!("Message type {:?}", message.message_type);
        }
    };
}

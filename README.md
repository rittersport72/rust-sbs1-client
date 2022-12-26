# rust-sbs1-client
This TCP client connects to dump1090. It decodes ADS-B messages in base station format SBS1 with the library https://github.com/samcrow/rust-sbs1.

SBS1 messages are in simple comma separated format and end with the "end of line" character. Therefore this TCP client reads data per line and not just any number of bytes:

```rust
// Create buffered reader from TcpStream
let mut reader = BufReader::new(stream);

// Create empty buffer
let mut buf = String::new();
        
// Check and read one line from TCP stream. This is a blocking call.
match reader.read_line(&mut buf) {
    ...
    }
```

Examples of some SBS1 messages:
```
MSG,5,111,11111,3C56EE,111111,2022/09/23,11:27:17.534,2022/09/23,11:27:17.513,,21900,,,,,,,0,,0,0
MSG,7,111,11111,A955D1,111111,2022/09/23,11:27:17.601,2022/09/23,11:27:17.579,,40025,,,,,,,,,,0
MSG,3,111,11111,A955D1,111111,2022/09/23,11:27:17.619,2022/09/23,11:27:17.580,,40025,,,48.44078,9.71170,,,,,,0
MSG,8,111,11111,A955D1,111111,2022/09/23,11:27:17.624,2022/09/23,11:27:17.580,,,,,,,,,,,,0
MSG,5,111,11111,3C56EE,111111,2022/09/23,11:27:17.625,2022/09/23,11:27:17.580,,21900,,,,,,,0,,0,0
MSG,4,111,11111,A955D1,111111,2022/09/23,11:27:17.627,2022/09/23,11:27:17.580,,,443,311,,,64,,,,,0
MSG,5,111,11111,A955D1,111111,2022/09/23,11:27:17.642,2022/09/23,11:27:17.580,,40025,,,,,,,0,,0,0
MSG,6,111,11111,A955D1,111111,2022/09/23,11:27:17.654,2022/09/23,11:27:17.644,,,,,,,,6655,0,0,0,0
```
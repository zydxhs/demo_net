use std::net::{TcpStream, ToSocketAddrs};
use std::str;
use std::{
    io::{self, prelude::*, BufReader},
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect_timeout(
        &"127.0.0.1:8080".to_socket_addrs().unwrap().next().unwrap(),
        Duration::from_secs(1),
    )?;

    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream.write(input.as_bytes()).expect("Failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Failed to read into buffer");
        println!("read from server: {}\n", str::from_utf8(&buffer).unwrap());
    }
    Ok(())
}

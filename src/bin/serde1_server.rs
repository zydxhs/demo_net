use serde::{Deserialize, Serialize};

use std::io::{self, prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("error: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| eprintln!("error: {}", e))
                });
            }
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);
    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let input: Point3D = serde_json::from_slice(&data)?;
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        let value = f64::from(value).sqrt();
        let v = serde_json::to_vec(&value)?;
        stream.get_mut().write(&v)?;
        stream.get_mut().write("\n".as_bytes())?;
        stream.get_mut().flush()?;
    }
}

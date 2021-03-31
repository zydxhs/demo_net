use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut handles = Vec::new();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handles.push(thread::spawn(move || {
                handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e))
            }));
        } else {
            eprintln!("error stream!");
            break;
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..100 {
        let bytes_read = stream.read(&mut buf)?;
        if 0 == bytes_read {
            break;
        }
        stream.write(&buf[0..bytes_read])?;
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

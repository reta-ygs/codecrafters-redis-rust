use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_connection(_stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let reader = std::io::BufReader::new(stream.try_clone()?);
    for _line in reader.lines() {
        stream.write("+PONG\r\n".as_bytes())?;
    }
    Ok(())
}

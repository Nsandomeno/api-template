use std::fmt::format;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream) {
    // Read first 16 characters from incoming stream.
    let mut buffer = [0; 16];
    stream.read(&mut buffer).await.unwrap();
    // First 4 characters are used to detect the HTTP Request Method
    let method_type = match str::from_utf8(&buffer[0..4]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let contents = match method_type {
        "GET" => {
            // TODO return real balance.
            format!("{{\"balance\": {}}}", 0.0)
        }
        "POST" => {
            // Take characters after 'POST /' until whitespace is detected.
            let input: String = buffer[6..16].iter()
                                    .take_while(|x| **x != 32u8)
                                    .map(|x| *x as char)
                                    .collect();
            let balance_update = input.parse::<f32>().unwrap();
            // TODO add balance update handling.
            format!("{{\"balance\": {}}}", balance_update)
        }
        _ => {
            panic!("Invalid HTTP Method!")
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // Handle multiple connections.
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}


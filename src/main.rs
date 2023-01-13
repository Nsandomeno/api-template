use std::str;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream, balance: Arc<Mutex<f32>>) {
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
            // balance needs to be locked before using it.
            format!("{{\"balance\": {}}}", balance.lock().unwrap())
        }
        "POST" => {
            // Take characters after 'POST /' until whitespace is detected.
            let input: String = buffer[6..16].iter()
                                    .take_while(|x| **x != 32u8)
                                    .map(|x| *x as char)
                                    .collect();
            let balance_update = input.parse::<f32>().unwrap();
            // Acquire the lock on the balance and mutate the value.
            let mut locked_balance = balance.lock().unwrap();
            *locked_balance += balance_update;
            
            format!("{{\"balance\": {}}}", locked_balance)
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
    // Create starting balance
    let balance = Arc::new(Mutex::new(0.00f32));
    let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // Clone the balance Arc and pass it to the handler.
        let balance = balance.clone();
        // Handle multiple connections.
        tokio::spawn(async move {
            handle_connection(stream, balance).await;
        });
    }
}


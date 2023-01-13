#[tokio::main]
async fn main() {
    let task = tokio::spawn(async {
        println!("Hello, rust!");
    });

    task.await.unwrap();
}


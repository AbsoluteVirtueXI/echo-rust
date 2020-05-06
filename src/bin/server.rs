use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    spawn_blocking(|| {
        println!("Hello from server.");
    });
}
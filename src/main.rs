#[tokio::main(worker_threads = 1)]
async fn main() {
    tripwire::app().await
}

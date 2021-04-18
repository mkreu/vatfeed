use vatfeed::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!(
        "{}",
        serde_json::to_string(&Downloader::init().download().await.unwrap()).unwrap()
    );
}

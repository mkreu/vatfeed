use vatfeed::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!(
        "{}",
        serde_json::to_string(
            &Downloader::with_status_file("status.json".into())
                .download()
                .await
                .unwrap()
        )
        .unwrap()
    );
}

use code_challenge::true_main;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    let args: Vec<String> = std::env::args().collect();

    match true_main(args).await {
        Ok(system) => {
            let csv = system.to_output().await;

        }
        Err(e) => {}
    }
}

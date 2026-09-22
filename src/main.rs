use code_challenge::true_main;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let _ = dotenvy::dotenv();

    // Setup loggin to error as we output to stdout
    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    let args: Vec<String> = std::env::args().collect();

    match true_main(args).await {
        Ok(system) => {
            let res = system.write(std::io::stdout()).await;
            if let Err(e) = res {
                tracing::error!("Failed to write system due to {e:?}")
            }
        }
        Err(e) => {
            tracing::error!("Failed to boot due to {e:?}")
        }
    }
}

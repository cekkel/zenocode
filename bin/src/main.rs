use zenocode_cli::start_cli;

#[tokio::main]
async fn main() {
    println!("Zenocode is here");
    start_cli().await;
}

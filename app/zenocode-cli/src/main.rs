use anyhow::Result;
use clap::Parser;
use zenocode_core::{get_provider, Config, CoreError};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Prompt to send to the AI
    prompt: String,

    /// Provider to use (default: openai)
    #[arg(short, long)]
    provider: Option<String>,

    /// Stream the response
    #[arg(short, long)]
    stream: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load config (simplified for now)
    let config = Config::load().map_err(|e| CoreError::ConfigError(e.to_string()))?;
    
    // Get provider from registry
    let provider_name = args.provider.as_deref().unwrap_or("openai");
    let provider = get_provider(provider_name, &config)
        .await
        .map_err(|e| CoreError::ProviderError(e.to_string()))?;

    if args.stream {
        let mut stream = provider.stream(&args.prompt).await?;
        while let Some(chunk) = stream.recv().await {
            print!("{}", chunk?);
        }
    } else {
        let response = provider.complete(&args.prompt).await?;
        println!("{}", response);
    }

    Ok(())
}
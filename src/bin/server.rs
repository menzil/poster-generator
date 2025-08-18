use clap::Parser;
use poster_generator::server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "3000", help = "Port to listen on")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    println!("Starting poster generator API server on port {}", cli.port);
    server::run_server(cli.port).await?;
    
    Ok(())
} 
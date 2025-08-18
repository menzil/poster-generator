use clap::Parser;
use poster_generator::PosterGenerator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "JSON config file for the poster")]
    config: PathBuf,

    #[arg(short, long, help = "Output file path")]
    output: PathBuf,

    #[arg(long, help = "Return base64 encoded image instead of file")]
    base64: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Read config file
    let config = std::fs::read_to_string(&cli.config)?;
    let config: poster_generator::PosterConfig = serde_json::from_str(&config)?;
    
    // Create poster generator
    let mut generator = PosterGenerator::new(config.width, config.height, config.background_color.clone());
    
    // Add elements from config
    for element in config.elements {
        match element {
            poster_generator::Element::Background(bg) => { generator.add_background(bg); },
            poster_generator::Element::Image(img) => { generator.add_image(img); },
            poster_generator::Element::Text(txt) => { generator.add_text(txt); },
        }
    }
    
    // Generate the poster
    if cli.base64 {
        let base64 = generator.generate_base64()?;
        println!("{}", base64);
    } else {
        generator.generate_file(&cli.output)?;
        println!("Poster saved to: {}", cli.output.display());
    }
    
    Ok(())
}

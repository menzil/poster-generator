use anyhow::Result;
use poster_generator::{
    BackgroundElement, Element, ImageElement, ObjectFit, PosterGenerator, Radius, TextAlignType,
    TextDirectionType, TextElement,
};

fn main() -> Result<()> {
    println!("Creating basic poster example...");

    // Create a poster generator
    let mut generator = PosterGenerator::new(800, 600, "#ffffff".to_string());

    // Add background
    let background = BackgroundElement {
        color: "#f5f5f5".to_string(),
        image: None,
        radius: Some(Radius::Single(20.0)),
    };
    generator.add_background(background);

    // Add title text
    let title = TextElement {
        text: "Hello, World!".to_string(),
        x: 400.0,
        y: 200.0,
        font_size: 64.0,
        color: "#333333".to_string(),
        align: TextAlignType::Center,
        bold: true,
        z_index: Some(2),
        ..Default::default()
    };
    generator.add_text(title);

    // Add subtitle
    let subtitle = TextElement {
        text: "A poster generation library with Skia Safe".to_string(),
        x: 400.0,
        y: 280.0,
        font_size: 24.0,
        color: "#666666".to_string(),
        align: TextAlignType::Center,
        max_width: Some(600.0),
        z_index: Some(2),
        ..Default::default()
    };
    generator.add_text(subtitle);

    // Add a price tag with background
    let price = TextElement {
        text: "99.99".to_string(),
        x: 400.0,
        y: 450.0,
        font_size: 48.0,
        color: "#ffffff".to_string(),
        align: TextAlignType::Center,
        prefix: Some("$".to_string()),
        background_color: Some("#ff6600".to_string()),
        padding: 20.0,
        border_radius: Some(Radius::Single(15.0)),
        bold: true,
        z_index: Some(3),
        ..Default::default()
    };
    generator.add_text(price);

    // Generate the poster
    generator.generate_file("examples/basic_output.png")?;
    println!("Basic poster saved to examples/basic_output.png");

    // Also generate as base64
    let base64 = generator.generate_base64()?;
    println!("Base64 (first 80 chars): {}", &base64[..80.min(base64.len())]);

    Ok(())
}

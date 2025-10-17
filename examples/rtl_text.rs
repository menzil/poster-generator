use anyhow::Result;
use poster_generator::{PosterGenerator, Radius, TextAlignType, TextDirectionType, TextElement};

fn main() -> Result<()> {
    println!("Creating RTL text poster example...");

    let mut generator = PosterGenerator::new(800, 700, "#f8f9fa".to_string());

    // Title in English
    let title = TextElement {
        text: "Multi-Language Poster Example".to_string(),
        x: 400.0,
        y: 60.0,
        font_size: 28.0,
        color: "#2c3e50".to_string(),
        align: TextAlignType::Center,
        bold: true,
        ..Default::default()
    };
    generator.add_text(title);

    // Chinese text
    let chinese = TextElement {
        text: "世界你好".to_string(),
        x: 400.0,
        y: 130.0,
        font_size: 36.0,
        color: "#f39c12".to_string(),
        align: TextAlignType::Center,
        font_family: Some("PingFang SC".to_string()),
        background_color: Some("#fff3e0".to_string()),
        padding: 10.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(chinese);

    // Arabic text
    let arabic = TextElement {
        text: "مرحبا بالعالم".to_string(),
        x: 400.0,
        y: 220.0,
        font_size: 40.0,
        color: "#e74c3c".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#ffe6e6".to_string()),
        padding: 12.0,
        border_radius: Some(Radius::Single(10.0)),
        ..Default::default()
    };
    generator.add_text(arabic);

    // Persian text
    let persian = TextElement {
        text: "سلام دنیا".to_string(),
        x: 400.0,
        y: 310.0,
        font_size: 38.0,
        color: "#3498db".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#e3f2fd".to_string()),
        padding: 10.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(persian);

    // Hebrew text
    let hebrew = TextElement {
        text: "שלום עולם".to_string(),
        x: 400.0,
        y: 400.0,
        font_size: 38.0,
        color: "#8e44ad".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#f3e5f5".to_string()),
        padding: 10.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(hebrew);

    // Uyghur text - using custom font file
    let uyghur = TextElement {
        text: "ئۇيغۇر تىلى، سالام دۇنيا".to_string(),
        x: 400.0,
        y: 490.0,
        font_size: 36.0,
        color: "#27ae60".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        font_family: Some("UKIJ Basma".to_string()),
        font_file: Some("UKIJBasma.ttf".to_string()), // Specify font file for Uyghur
        background_color: Some("#e8f5e9".to_string()),
        padding: 10.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(uyghur);

    // Footer note
    let footer = TextElement {
        text: "Supports Chinese, Arabic, Persian, Hebrew, and Uyghur".to_string(),
        x: 400.0,
        y: 650.0,
        font_size: 14.0,
        color: "#7f8c8d".to_string(),
        align: TextAlignType::Center,
        ..Default::default()
    };
    generator.add_text(footer);

    // Generate the poster
    generator.generate_file("examples/rtl_output.png")?;
    println!("RTL text poster saved to examples/rtl_output.png");

    Ok(())
}

use anyhow::Result;
use poster_generator::{
    PosterGenerator, TextAlignType, TextDirectionType, TextElement, Radius,
};

fn main() -> Result<()> {
    println!("Creating RTL text poster example...");

    let mut generator = PosterGenerator::new(800, 600, "#f8f9fa".to_string());

    // Title in English
    let title = TextElement {
        text: "Multi-Language Poster Example".to_string(),
        x: 400.0,
        y: 80.0,
        font_size: 32.0,
        color: "#2c3e50".to_string(),
        align: TextAlignType::Center,
        bold: true,
        ..Default::default()
    };
    generator.add_text(title);

    // Arabic text
    let arabic = TextElement {
        text: "مرحبا بالعالم".to_string(),
        x: 400.0,
        y: 180.0,
        font_size: 48.0,
        color: "#e74c3c".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#ffe6e6".to_string()),
        padding: 15.0,
        border_radius: Some(Radius::Single(10.0)),
        ..Default::default()
    };
    generator.add_text(arabic);

    // Persian text
    let persian = TextElement {
        text: "سلام دنیا".to_string(),
        x: 400.0,
        y: 280.0,
        font_size: 42.0,
        color: "#3498db".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#e3f2fd".to_string()),
        padding: 12.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(persian);

    // Uyghur text
    let uyghur = TextElement {
        text: "ئۇيغۇر تىلى، سالام دۇنيا".to_string(),
        x: 400.0,
        y: 380.0,
        font_size: 40.0,
        color: "#27ae60".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        font_family: Some("ALKATIP Basma Tom".to_string()),
        background_color: Some("#e8f5e9".to_string()),
        padding: 12.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(uyghur);

    // Hebrew text
    let hebrew = TextElement {
        text: "שלום עולם".to_string(),
        x: 400.0,
        y: 480.0,
        font_size: 42.0,
        color: "#8e44ad".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#f3e5f5".to_string()),
        padding: 12.0,
        border_radius: Some(Radius::Single(8.0)),
        ..Default::default()
    };
    generator.add_text(hebrew);

    // Footer note
    let footer = TextElement {
        text: "RTL text is automatically detected and rendered correctly".to_string(),
        x: 400.0,
        y: 560.0,
        font_size: 16.0,
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

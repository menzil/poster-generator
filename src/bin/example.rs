use anyhow::Result;
use poster_generator::{
    BackgroundElement, Element, ImageElement, ObjectFit, PosterConfig, PosterGenerator, Radius,
    TextAlignType, TextDirectionType, TextElement,
};

fn main() -> Result<()> {
    println!("创建示例海报...");

    // 创建一个简单的海报配置
    let config = PosterConfig {
        width: 750,
        height: 600,
        background_color: "#ffffff".to_string(),
        elements: vec![
            Element::Background(BackgroundElement {
                color: "#f5f5f5".to_string(),
                image: None,
                radius: Some(Radius::Single(20.0)),
            }),
            Element::Image(ImageElement {
                src: "sample_image.jpg".to_string(), // 请替换为实际存在的图片路径
                x: 50.0,
                y: 50.0,
                width: 650.0,
                height: 300.0,
                radius: Some(Radius::Single(10.0)),
                z_index: Some(1),
                object_fit: ObjectFit::Cover,
            }),
            Element::Text(TextElement {
                text: "使用 Skia Safe 的海报生成器".to_string(),
                x: 375.0,
                y: 400.0,
                font_size: 40.0,
                color: "#333333".to_string(),
                align: TextAlignType::Center,
                font_family: None,
                font_file: None,
                max_width: None,
                line_height: 1.5,
                max_lines: None,
                z_index: Some(2),
                bold: true,
                prefix: None,
                background_color: None,
                padding: 0.0,
                border_radius: None,
                width: None,
                height: None,
                direction: TextDirectionType::Ltr,
            }),
            Element::Text(TextElement {
                text: "这是一个使用 Skia Safe 库实现的海报生成工具的示例，支持多行文本、图片、圆角等功能。".to_string(),
                x: 375.0,
                y: 450.0,
                font_size: 24.0,
                color: "#666666".to_string(),
                align: TextAlignType::Center,
                font_family: None,
                font_file: None,
                max_width: Some(600.0),
                line_height: 1.5,
                max_lines: Some(3),
                z_index: Some(2),
                bold: false,
                prefix: None,
                background_color: None,
                padding: 0.0,
                border_radius: None,
                width: None,
                height: None,
                direction: TextDirectionType::Ltr,
            }),
            Element::Text(TextElement {
                text: "价格: 99.99".to_string(),
                x: 375.0,
                y: 550.0,
                font_size: 32.0,
                color: "#ffffff".to_string(),
                align: TextAlignType::Center,
                font_family: None,
                font_file: None,
                max_width: None,
                line_height: 1.5,
                max_lines: None,
                z_index: Some(3),
                bold: false,
                prefix: Some("¥".to_string()),
                background_color: Some("#ff6600".to_string()),
                padding: 10.0,
                border_radius: Some(Radius::Single(15.0)),
                width: None,
                height: None,
                direction: TextDirectionType::Ltr,
            }),
            // 添加一个RTL方向的文本元素(维吾尔语示例) - using custom font file
            Element::Text(TextElement {
                text: "ياخشىمۇ مەن كەلدىم".to_string(), // 维吾尔语"你好，我来了"
                x: 375.0,
                y: 500.0,
                font_size: 28.0,
                color: "#0066cc".to_string(),
                align: TextAlignType::Right, // 对于RTL文本，通常使用右对齐
                font_family: None,
                font_file: Some("UKIJBasma.ttf".to_string()), // 指定维吾尔语字体文件
                max_width: Some(600.0),
                line_height: 1.5,
                max_lines: None,
                z_index: Some(3),
                bold: false,
                prefix: None,
                background_color: Some("#e6f7ff".to_string()),
                padding: 8.0,
                border_radius: Some(Radius::Single(8.0)),
                width: None,
                height: None,
                direction: TextDirectionType::Rtl, // 设置为RTL方向
            }),
        ],
    };

    // 创建海报生成器
    let mut generator =
        PosterGenerator::new(config.width, config.height, config.background_color.clone());

    // 设置元素
    generator.set_elements(config.elements);

    // 生成海报并保存
    println!("正在生成海报文件...");
    generator.generate_file("example_output.png")?;
    println!("海报已保存为 example_output.png");

    // 生成 base64 编码
    println!("正在生成 base64 编码...");
    let base64 = generator.generate_base64()?;
    println!(
        "Base64 编码 (前100个字符): {}",
        &base64[..100.min(base64.len())]
    );

    Ok(())
}

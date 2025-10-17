# Poster Generator

[![Crates.io](https://img.shields.io/crates/v/poster_generator.svg)](https://crates.io/crates/poster_generator)
[![Documentation](https://docs.rs/poster_generator/badge.svg)](https://docs.rs/poster_generator)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

一个基于 Rust 和 Skia Safe 的海报生成库，支持 RTL (从右到左) 文本渲染，适用于阿拉伯语、希伯来语、波斯语和维吾尔语等语言。

A poster generation library based on Skia Safe with RTL (Right-to-Left) text support for Arabic, Hebrew, Persian, and Uyghur languages.

## 特性 Features

- ✅ 创建可配置宽高的海报 / Customizable canvas size
- ✅ 背景元素（颜色或图片），支持圆角 / Background with colors, images, and rounded corners
- ✅ 图片元素：
  - 位置设置（x, y 坐标）/ Positioning (x, y coordinates)
  - 尺寸设置（宽度，高度）/ Sizing (width, height)
  - 圆角 / Rounded corners
  - 图片适应模式（cover, contain, stretch）/ Object fit modes
  - z-index 层级 / Z-index layering
- ✅ 文本元素：
  - 位置和尺寸设置 / Positioning and sizing
  - 字体样式（大小，颜色，加粗）/ Font styling (size, color, bold)
  - 文本对齐方式（左对齐，居中，右对齐）/ Text alignment (left, center, right)
  - 多行文本自动换行和行高控制 / Multi-line text with automatic wrapping
  - 最大行数限制，超出自动添加省略号 / Max lines with ellipsis
  - 文本背景色，内边距和圆角 / Text background with padding and border radius
  - z-index 层级 / Z-index layering
  - **支持从右往左(RTL)的文本渲染** / **RTL text rendering support**
- ✅ 输出为 PNG 文件或 base64 编码字符串 / Export as PNG file or base64 string

## 安装 Installation

### 作为库使用 As a Library

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
poster_generator = "0.1"
```

或使用 cargo add:

```bash
cargo add poster_generator
```

### 命令行工具 CLI Tool

```bash
cargo install poster_generator
```

## 使用方法 Usage

### 作为库 As a Library

```rust
use poster_generator::{PosterGenerator, TextElement, TextAlignType, TextDirectionType};

fn main() -> anyhow::Result<()> {
    // 创建生成器
    let mut generator = PosterGenerator::new(800, 600, "#ffffff".to_string());

    // 添加文本元素
    let text = TextElement {
        text: "Hello, World!".to_string(),
        x: 400.0,
        y: 300.0,
        font_size: 48.0,
        color: "#333333".to_string(),
        align: TextAlignType::Center,
        ..Default::default()
    };

    generator.add_text(text);

    // 生成并保存
    generator.generate_file("output.png")?;

    Ok(())
}
```

### RTL 文本示例 RTL Text Example

```rust
use poster_generator::{PosterGenerator, TextElement, TextAlignType, TextDirectionType};

fn main() -> anyhow::Result<()> {
    let mut generator = PosterGenerator::new(800, 600, "#f8f9fa".to_string());

    // 维吾尔语文本
    let uyghur_text = TextElement {
        text: "ئۇيغۇر تىلى".to_string(),
        x: 400.0,
        y: 200.0,
        font_size: 48.0,
        color: "#2c3e50".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        font_family: Some("ALKATIP Basma Tom".to_string()),
        ..Default::default()
    };

    // 阿拉伯语文本
    let arabic_text = TextElement {
        text: "مرحبا بالعالم".to_string(),
        x: 400.0,
        y: 300.0,
        font_size: 36.0,
        color: "#0066cc".to_string(),
        align: TextAlignType::Center,
        direction: TextDirectionType::Rtl,
        background_color: Some("#e6f7ff".to_string()),
        padding: 10.0,
        ..Default::default()
    };

    generator.add_text(uyghur_text);
    generator.add_text(arabic_text);

    generator.generate_file("rtl_poster.png")?;

    Ok(())
}
```

### 命令行使用 CLI Usage

```bash
# 从 JSON 配置文件生成海报
poster_generator -c config.json -o output.png

# 生成 base64 输出
poster_generator -c config.json -o output.png --base64

# 运行示例
poster_generator_example
```

## JSON 配置格式 JSON Configuration

```json
{
  "width": 800,
  "height": 600,
  "background_color": "#ffffff",
  "elements": [
    {
      "type": "background",
      "color": "#f5f5f5",
      "radius": 20
    },
    {
      "type": "image",
      "src": "photo.jpg",
      "x": 50,
      "y": 50,
      "width": 700,
      "height": 400,
      "radius": 10,
      "object_fit": "cover",
      "z_index": 1
    },
    {
      "type": "text",
      "text": "Hello, World!",
      "x": 400,
      "y": 500,
      "font_size": 48,
      "color": "#333333",
      "align": "center",
      "bold": true,
      "z_index": 2
    }
  ]
}
```

## RTL 文本支持 RTL Text Support

本库自动检测并支持以下语言的 RTL 文本渲染：

This library automatically detects and supports RTL text rendering for:

- **阿拉伯语 Arabic**: مرحبا بالعالم
- **波斯语 Persian**: سلام دنیا
- **维吾尔语 Uyghur**: ئۇيغۇر تىلى، سالام دۇنيا
- **希伯来语 Hebrew**: שלום עולם

### RTL 配置要点 RTL Configuration Tips

1. **自动检测 Auto-detection**: 系统会自动检测文本中的 RTL 字符 / System automatically detects RTL characters
2. **手动指定 Manual specification**: 可通过 `direction: "rtl"` 手动指定 / Use `direction: "rtl"` to manually specify
3. **字体选择 Font selection**: 系统自动选择支持的字体，也可通过 `font_family` 指定 / System auto-selects fonts, or specify via `font_family`
4. **对齐方式 Alignment**: RTL 文本通常使用 `align: "right"` 或 `align: "center"` / RTL text usually uses right or center alignment

## 依赖 Dependencies

- **skia-safe**: Skia 图形库的 Rust 绑定 / Rust bindings for Skia graphics library
- **serde/serde_json**: JSON 序列化支持 / JSON serialization
- **base64**: Base64 编码支持 / Base64 encoding
- **anyhow/thiserror**: 错误处理 / Error handling

## 文档 Documentation

完整 API 文档请访问 [docs.rs](https://docs.rs/poster_generator)

For complete API documentation, visit [docs.rs](https://docs.rs/poster_generator)

## 许可证 License

MIT License - 详见 [LICENSE](LICENSE) 文件

MIT License - See [LICENSE](LICENSE) file for details

## 贡献 Contributing

欢迎贡献！请随时提交 Pull Request。

Contributions are welcome! Feel free to submit a Pull Request.

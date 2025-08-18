# Poster Generator

一个基于 Rust 和 Skia Safe 的海报生成工具，可以创建包含文本和图像元素的海报图片。

## 特性

- 创建可配置宽高的海报
- 添加背景元素（颜色或图片），支持圆角
- 添加图片元素，支持：
  - 位置设置（x, y 坐标）
  - 尺寸设置（宽度，高度）
  - 圆角
  - 图片适应模式（cover, contain, stretch）
  - z-index 层级
- 添加文本元素，支持：
  - 位置和尺寸设置
  - 字体样式（大小，颜色，加粗）
  - 文本对齐方式（左对齐，居中，右对齐）
  - 多行文本自动换行和行高控制
  - 最大行数限制，超出自动添加省略号
  - 文本背景色，内边距和圆角
  - z-index 层级
  - 支持从右往左(RTL)的文本渲染，适用于阿拉伯语、希伯来语等
- 输出为 PNG 文件或 base64 编码字符串
- 命令行界面和 HTTP API

## 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/poster_generator.git
cd poster_generator

# 构建项目
cargo build --release
```

## 使用方法

### 命令行

```bash
# 从 JSON 配置文件生成海报
cargo run --release --bin poster_generator -- -c example_config.json -o output_poster.png

# 生成海报并输出为 base64
cargo run --release --bin poster_generator -- -c example_config.json --base64

# 运行示例
cargo run --release --bin example
```

### API 服务器

```bash
# 启动 API 服务器
cargo run --release --bin server -- -p 3000
```

然后可以向 `http://localhost:3000/generate` 发送 POST 请求，请求体如下：

```json
{
  "config": {
    "width": 750,
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
        "src": "https://example.com/image.jpg",
        "x": 50,
        "y": 50,
        "width": 650,
        "height": 300,
        "radius": 10,
        "object_fit": "cover",
        "z_index": 1
      },
      {
        "type": "text",
        "text": "海报标题示例",
        "x": 375,
        "y": 400,
        "font_size": 40,
        "color": "#333333",
        "align": "center",
        "bold": true,
        "z_index": 2,
        "direction": "ltr"
      },
      {
        "type": "text",
        "text": "مرحبا بالعالم",
        "x": 375,
        "y": 500,
        "font_size": 30,
        "color": "#0066cc",
        "align": "right",
        "bold": false,
        "background_color": "#e6f7ff", 
        "padding": 8,
        "z_index": 2,
        "direction": "rtl"
      }
    ]
  },
  "format": "base64"
}
```

API 将返回：

```json
{
  "success": true,
  "data": "data:image/png;base64,iVBORw0KGgoAAAANSUhEU...",
  "error": null
}
```

## 配置

海报生成器使用 JSON 配置格式：

```json
{
  "width": 750,
  "height": 600,
  "background_color": "#ffffff",
  "elements": [
    // 元素数组（背景、图片、文本）
  ]
}
```

### 元素类型

#### 背景

```json
{
  "type": "background",
  "color": "#f5f5f5",
  "image": "可选图片路径.jpg",
  "radius": 20
}
```

#### 图片

```json
{
  "type": "image",
  "src": "图片路径.jpg",
  "x": 50,
  "y": 50,
  "width": 300,
  "height": 200,
  "radius": 10,
  "object_fit": "cover",
  "z_index": 1
}
```

#### 文本

```json
{
  "type": "text",
  "text": "你好，世界！",
  "x": 100,
  "y": 100,
  "font_size": 24,
  "color": "#333333",
  "align": "left",
  "font_family": "Arial",  // 可选：指定字体族，如"ALKATIP Basma Tom"用于维吾尔语
  "max_width": 400,
  "line_height": 1.5,
  "max_lines": 2,
  "bold": false,
  "prefix": "¥",
  "background_color": "#f0f0f0",
  "padding": 10,
  "border_radius": 5,
  "z_index": 1,
  "direction": "ltr"  // 可选值: "ltr"(从左到右) 或 "rtl"(从右到左)
}
```

## RTL 文本与多语言支持

本工具支持从右到左(RTL)的文本方向，适用于阿拉伯语、希伯来语、波斯语、维吾尔语等语言。系统会自动检测RTL文字并选择合适的字体。

### 支持的语言
- **阿拉伯语**：مرحبا بالعالم
- **波斯语**：سلام دنیا
- **维吾尔语**：ئۇيغۇر تىلى، سالام دۇنيا
- **希伯来语**：שלום עולם
- **中文**：你好，世界
- **英语**：Hello, World
- **其他拉丁字母语言**

### RTL文本配置

RTL文本的渲染需要以下设置：

1. **自动检测**：系统会自动检测文本中的RTL字符，无需手动设置
2. **手动指定**：也可以通过 `direction: "rtl"` 手动指定文本方向
3. **字体选择**：系统会自动为RTL文本选择支持该语言的字体，也可通过 `font_family` 指定专用字体
4. **对齐方式**：通常RTL文本使用 `align: "right"` 或 `align: "center"`
5. **连字支持**：保持原始文本顺序以支持正确的阿拉伯文字连字渲染

### 维吾尔语示例

```json
{
  "type": "text",
  "text": "ئۇيغۇر تىلى",
  "x": 400,
  "y": 150,
  "font_size": 48,
  "color": "#2c3e50",
  "align": "center",
  "direction": "rtl",
  "font_family": "ALKATIP Basma Tom",
  "bold": true
}
```

### 阿拉伯语示例

```json
{
  "type": "text",
  "text": "مرحبا بالعالم",
  "x": 375,
  "y": 500,
  "font_size": 30,
  "color": "#0066cc",
  "align": "right",
  "direction": "rtl",
  "background_color": "#e6f7ff",
  "padding": 8
}
```

### 混合语言文档

```json
{
  "width": 800,
  "height": 600,
  "background_color": "#f8f9fa",
  "elements": [
    {
      "type": "text",
      "text": "ئۇيغۇر تىلى",
      "x": 400,
      "y": 150,
      "font_size": 36,
      "color": "#e74c3c",
      "align": "center",
      "direction": "rtl"
    },
    {
      "type": "text",
      "text": "Uyghur Language / 维吾尔语",
      "x": 400,
      "y": 200,
      "font_size": 24,
      "color": "#3498db",
      "align": "center",
      "direction": "ltr"
    }
  ]
}
```

## 特点

本项目使用 Skia Safe（Skia 图形库的 Rust 绑定）进行渲染，相比原先的 tiny-skia 库提供了以下优势：

1. 真实的文本渲染支持，而不是简单的占位符
2. 对RTL文本的原生支持
3. 更好的图像处理能力
4. 丰富的图形绘制功能
5. 与 Chrome、Android 等主流平台使用相同的渲染引擎

## 依赖

- skia-safe：Skia 图形库的 Rust 安全绑定
- base64：处理 base64 编码和解码
- image：图像处理
- serde/serde_json：序列化和反序列化 JSON
- axum：Web API 框架
- clap：命令行参数解析
- tokio：异步运行时
- anyhow/thiserror：错误处理

## 许可证

MIT 
# 维吾尔文字支持 / Uyghur Text Support

本海报生成器提供对维吾尔文字的全面支持，包括正确的从右到左（RTL）文本方向和字体选择。

## 问题解决

### 维吾尔文字显示问题的常见解决方案

如果维吾尔文字显示不正确（方向不对、字体不好看或连字问题），系统提供了以下改进：

#### 1. 自动RTL检测
- 系统会自动检测维吾尔文字的Unicode范围（Arabic script）
- 无需手动设置，系统自动应用RTL文本处理

#### 2. 字体族指定功能
现在可以通过 `font_family` 参数指定特定字体，支持维吾尔文字连字和正确显示：

**推荐的维吾尔语字体（按优先级）：**
```
1. "ALKATIP Basma Tom" - 专业维吾尔语字体，支持完整连字
2. "ALKATIPBasmaTom" - 无空格版本  
3. "Noto Naskh Arabic" - Google Arabic字体，连字支持良好
4. "Amiri" - 传统阿拉伯字体，连字优秀
5. "Scheherazade New" - SIL字体，维吾尔语支持好
6. "Arabic Typesetting" - Microsoft阿拉伯字体
7. "Geeza Pro" - macOS阿拉伯字体
8. "Al Bayan" - macOS阿拉伯字体
```

#### 3. 智能字体回退
如果指定的字体不可用，系统会按以下顺序回退：
- 用户指定字体 → RTL专用字体列表 → 系统默认字体

#### 4. 连字支持
- 不再反转文本字符顺序，保持原始文本以支持正确的连字渲染
- 让Skia引擎处理正确的RTL方向和字形连接

#### 5. 方向自动调整
- 对于维吾尔文字，系统会自动反转对齐方向
- `align: "left"` 在RTL文本中会表现为右对齐
- `align: "right"` 在RTL文本中会表现为左对齐
- `align: "center"` 保持居中对齐

## 使用示例

### 基本维吾尔语文本（自动字体选择）
```json
{
  "type": "text",
  "text": "ئۇيغۇر تىلى",
  "x": 400,
  "y": 150,
  "font_size": 36,
  "color": "#2c3e50",
  "align": "center"
}
```

### 指定维吾尔语专用字体
```json
{
  "type": "text",
  "text": "ئۇيغۇر تىلى",
  "x": 400,
  "y": 150,
  "font_size": 36,
  "color": "#2c3e50",
  "align": "center",
  "font_family": "ALKATIP Basma Tom",
  "direction": "rtl"
}
```

### 维吾尔语问候语（使用指定字体）
```json
{
  "type": "text",
  "text": "سالام دۇنيا",
  "x": 400,
  "y": 200,
  "font_size": 28,
  "color": "#e74c3c",
  "align": "center",
  "direction": "rtl",
  "font_family": "ALKATIP Basma Tom"
}
```

### 带背景的维吾尔语文本
```json
{
  "type": "text",
  "text": "مۇرەككەپ تېكىست ئۈچۈن سىناق",
  "x": 400,
  "y": 300,
  "font_size": 24,
  "color": "#27ae60",
  "align": "center",
  "max_width": 600,
  "background_color": "#ecf0f1",
  "padding": 15,
  "border_radius": 10,
  "direction": "rtl"
}
```

### 多行维吾尔语文本
```json
{
  "type": "text",
  "text": "بۇ بىر ئۇزۇن تېكىست مىسالى، ئۇ كۆپ قۇرغا بۆلۈنىدۇ",
  "x": 400,
  "y": 400,
  "font_size": 20,
  "color": "#3498db",
  "align": "right",
  "max_width": 500,
  "max_lines": 3,
  "line_height": 1.6,
  "direction": "rtl"
}
```

## 完整的维吾尔语海报示例

```json
{
  "width": 800,
  "height": 600,
  "background_color": "#f8f9fa",
  "elements": [
    {
      "type": "background",
      "color": "#ffffff",
      "radius": 20
    },
    {
      "type": "text",
      "text": "ئۇيغۇر تىلى",
      "x": 400,
      "y": 150,
      "font_size": 48,
      "color": "#2c3e50",
      "align": "center",
      "direction": "rtl",
      "bold": true,
      "z_index": 1
    },
    {
      "type": "text",
      "text": "سالام دۇنيا",
      "x": 400,
      "y": 220,
      "font_size": 32,
      "color": "#e74c3c",
      "align": "center",
      "direction": "rtl",
      "z_index": 1
    },
    {
      "type": "text",
      "text": "Hello World (English)",
      "x": 400,
      "y": 280,
      "font_size": 24,
      "color": "#3498db",
      "align": "center",
      "direction": "ltr",
      "z_index": 1
    },
    {
      "type": "text",
      "text": "مۇرەككەپ تېكىست ئۈچۈن سىناق",
      "x": 400,
      "y": 350,
      "font_size": 20,
      "color": "#27ae60",
      "align": "center",
      "direction": "rtl",
      "max_width": 600,
      "background_color": "#ecf0f1",
      "padding": 10,
      "border_radius": 10,
      "z_index": 2
    }
  ]
}
```

## 技术实现

### Unicode 支持
系统支持以下Unicode范围的维吾尔文字：
- Arabic (U+0600-U+06FF)
- Arabic Supplement (U+0750-U+077F)
- Arabic Extended-A (U+08A0-U+08FF)
- Arabic Presentation Forms-A (U+FB50-U+FDFF)
- Arabic Presentation Forms-B (U+FE70-U+FEFF)

### 字符处理
- 自动检测RTL字符
- 基本的字符顺序处理（简化版BiDi算法）
- 适当的文本测量和布局

### 字体匹配
- 优先选择支持阿拉伯文字的字体
- 字体粗体/正常样式的正确应用
- 回退到系统默认字体

## 故障排除

### 如果维吾尔文字仍然显示不正确

#### 1. 检查字体安装
确保系统安装了支持阿拉伯/维吾尔文字的字体：

**macOS系统检查：**
```bash
# 查看系统中可用的阿拉伯文字体
fc-list :lang=ar family
```

**推荐安装的字体：**
- **ALKATIP字体系列** - 专门的维吾尔语字体
- **Noto Sans Arabic** - Google开源字体，支持完整阿拉伯文
- **Amiri** - 传统阿拉伯字体，连字支持优秀
- **Scheherazade** - SIL字体，维吾尔语支持好

#### 2. 字体安装方法

**macOS系统字体安装：**
1. 下载字体文件（.ttf 或 .otf）
2. 双击字体文件
3. 点击"安装字体"
4. 重启应用程序

**或者复制到字体目录：**
```bash
# 用户字体目录
cp font.ttf ~/Library/Fonts/

# 系统字体目录（需要管理员权限）
sudo cp font.ttf /Library/Fonts/
```

#### 3. 测试字体是否正确加载
使用简单的测试配置：

```json
{
  "width": 400,
  "height": 200,
  "background_color": "#ffffff",
  "elements": [
    {
      "type": "text",
      "text": "سالام",
      "x": 200,
      "y": 100,
      "font_size": 48,
      "color": "#000000",
      "align": "center",
      "direction": "rtl",
      "font_family": "Arial Unicode MS"
    }
  ]
}
```

#### 4. 常见问题及解决方案

**问题：文字显示为方块或问号**
- 原因：系统缺少支持该Unicode范围的字体
- 解决：安装阿拉伯文字体，或使用 `"font_family": "Arial Unicode MS"`

**问题：文字方向错误（从左到右显示）**
- 原因：未正确设置RTL方向或字体不支持RTL
- 解决：确保设置 `"direction": "rtl"` 并使用支持RTL的字体

**问题：字符不连续（连字断裂）**  
- 原因：字体不支持阿拉伯文连字或使用了不当的文本处理
- 解决：使用专业阿拉伯文字体如 "Amiri"、"Geeza Pro" 等

**问题：字体指定无效**
- 原因：指定的字体名称不正确或未安装
- 解决：使用系统字体查看工具确认正确的字体名称

#### 5. 推荐的字体配置

**最佳实践配置：**
```json
{
  "type": "text",
  "text": "ئۇيغۇر تىلى - سالام دۇنيا",
  "x": 400,
  "y": 200,
  "font_size": 32,
  "color": "#2c3e50",
  "align": "center",
  "direction": "rtl",
  "font_family": "Geeza Pro"
}
```

**备选方案（如果专用字体不可用）：**
```json
{
  "font_family": "Arial Unicode MS"
}
```

## 限制与改进空间

当前实现提供了基础但实用的RTL文本支持。对于更高级的需求，建议：

1. **完整的BiDi支持**：使用专门的Unicode BiDi库如ICU
2. **字形塑造**：集成HarfBuzz进行专业文本塑造
3. **更多字体**：安装专业维吾尔语/阿拉伯文字体集合
4. **复杂文本布局**：支持更复杂的文本布局和排版需求
5. **文字渲染引擎升级**：考虑使用更高级的文本渲染引擎

## 生成命令

```bash
# 生成维吾尔语海报
cargo run --release -- -c uyghur_example.json -o uyghur_poster.png

# 生成base64输出
cargo run --release -- -c uyghur_example.json --base64 -o output.png
```

通过这些改进，维吾尔文字现在应该可以正确显示，方向正确，字体也更加美观。
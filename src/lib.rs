use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::path::Path;
use skia_safe::{
    Canvas, Color, Data, EncodedImageFormat, Font, 
    FontMgr, FontStyle, Image, Paint, Path as SkPath, Point, Rect, 
    Surface, TextBlob, Typeface,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextDirection, TextStyle, TypefaceFontProvider}
};
use thiserror::Error;

// Custom error type
#[derive(Error, Debug)]
pub enum PosterError {
    #[error("Failed to load image: {0}")]
    ImageLoadError(String),
    
    #[error("Failed to render element: {0}")]
    RenderError(String),
    
    #[error("Failed to generate output: {0}")]
    OutputError(String),
}

// Main config structure
#[derive(Debug, Deserialize, Serialize)]
pub struct PosterConfig {
    pub width: u32,
    pub height: u32,
    pub background_color: String,
    pub elements: Vec<Element>,
}

// Element types
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "background")]
    Background(BackgroundElement),
    
    #[serde(rename = "image")]
    Image(ImageElement),
    
    #[serde(rename = "text")]
    Text(TextElement),
}

// Background element
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackgroundElement {
    pub image: Option<String>,
    pub color: String,
    pub radius: Option<Radius>,
}

// Image element
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageElement {
    pub src: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub radius: Option<Radius>,
    pub z_index: Option<i32>,
    #[serde(default = "default_object_fit")]
    pub object_fit: ObjectFit,
}

// Text element
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TextElement {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    pub color: String,
    #[serde(default = "default_text_align")]
    pub align: TextAlignType,
    pub font_family: Option<String>,
    pub max_width: Option<f32>,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
    pub max_lines: Option<u32>,
    pub z_index: Option<i32>,
    #[serde(default = "default_bold")]
    pub bold: bool,
    pub prefix: Option<String>,
    pub background_color: Option<String>,
    #[serde(default = "default_padding")]
    pub padding: f32,
    pub border_radius: Option<Radius>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    #[serde(default = "default_text_direction")]
    pub direction: TextDirectionType,
}

// Radius type can be a single value or an array for each corner
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Radius {
    Single(f32),
    Multiple([f32; 4]), // top-left, top-right, bottom-right, bottom-left
}

// Object fit enum
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ObjectFit {
    Cover,
    Contain,
    Stretch,
}

// Text alignment enum
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextAlignType {
    Left,
    Center,
    Right,
}

// Text direction enum
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TextDirectionType {
    Ltr,
    Rtl,
}

// Utility function to detect RTL/Arabic script text
fn is_rtl_text(text: &str) -> bool {
    // Check for Arabic/Persian/Uyghur Unicode ranges
    text.chars().any(|c| {
        let code = c as u32;
        // Arabic: U+0600-U+06FF
        // Arabic Supplement: U+0750-U+077F
        // Arabic Extended-A: U+08A0-U+08FF
        // Arabic Presentation Forms-A: U+FB50-U+FDFF
        // Arabic Presentation Forms-B: U+FE70-U+FEFF
        (code >= 0x0600 && code <= 0x06FF) ||
        (code >= 0x0750 && code <= 0x077F) ||
        (code >= 0x08A0 && code <= 0x08FF) ||
        (code >= 0xFB50 && code <= 0xFDFF) ||
        (code >= 0xFE70 && code <= 0xFEFF)
    })
}

// Function to load font from file
fn load_font_from_file(font_path: &str, font_size: f32) -> Option<Font> {
    if let Ok(font_data) = std::fs::read(font_path) {
        let font_mgr = FontMgr::new();
        if let Some(typeface) = font_mgr.new_from_data(&font_data, None) {
            return Some(Font::new(typeface, font_size));
        }
    }
    None
}

// Function to get appropriate font for text with optional font family
fn get_font_for_text_with_family(text: &str, font_size: f32, bold: bool, font_family: Option<&str>) -> Font {
    let font_mgr = FontMgr::default();
    
    let weight = if bold { 
        skia_safe::font_style::Weight::BOLD 
    } else { 
        skia_safe::font_style::Weight::NORMAL 
    };
    
    let font_style = FontStyle::new(weight, skia_safe::font_style::Width::NORMAL, skia_safe::font_style::Slant::Upright);
    
    // For RTL text, try loading UKIJBasma font from file first
    if is_rtl_text(text) {
        // Try to load UKIJBasma font from local file
        if let Some(font) = load_font_from_file("UKIJBasma.ttf", font_size) {
            return font;
        }
        if let Some(font) = load_font_from_file("./UKIJBasma.ttf", font_size) {
            return font;
        }
    }
    
    // If user specified a font family, try that next
    if let Some(family) = font_family {
        if let Some(typeface) = font_mgr.match_family_style(family, font_style) {
            return Font::new(typeface, font_size);
        }
    }
    
    // For RTL/Arabic scripts including Uyghur, prioritize UKIJBasma and other Arabic fonts
    let font_families = if is_rtl_text(text) {
        // Priority order: UKIJBasma first (专门的维吾尔语字体), then other Arabic fonts
        vec![
            "UKIJBasma",              // 专门的维吾尔语字体 - 最高优先级
            "UKIJ Basma",             // 可能的替代名称
            "Geeza Pro",              // macOS Arabic font - excellent RTL support
            "Al Bayan",               // macOS Arabic font - good for Uyghur
            "Arial Unicode MS",       // Comprehensive Unicode coverage
            "Baghdad",                // macOS Arabic font
            "Nadeem",                 // macOS Arabic font
            "DejaVu Sans",            // Open source with Arabic support
            "Times New Roman",        // Has some Arabic glyphs
            "Arial",                  // Basic fallback
            "Helvetica"               // System fallback
        ]
    } else {
        vec![
            "SF Pro Text",     // macOS system font
            "Arial",           // Cross-platform
            "Helvetica",       // macOS standard
            "Times New Roman", // Classic fallback
        ]
    };
    
    // Try to find a suitable font
    for family in font_families {
        if let Some(typeface) = font_mgr.match_family_style(family, font_style) {
            return Font::new(typeface, font_size);
        }
    }
    
    // Fallback to default font
    let font_mgr = FontMgr::default();
    if let Some(typeface) = font_mgr.legacy_make_typeface(None, FontStyle::normal()) {
        Font::new(typeface, font_size)
    } else {
        // Last resort - create a font from system default typeface
        let system_mgr = FontMgr::new();
        if let Some(default_typeface) = system_mgr.legacy_make_typeface(None, FontStyle::normal()) {
            Font::new(default_typeface, font_size)
        } else {
            // Very last resort - use built-in default
            Font::default()
        }
    }
}

// Function to get appropriate font for text (backward compatibility)
fn get_font_for_text(text: &str, font_size: f32, bold: bool) -> Font {
    get_font_for_text_with_family(text, font_size, bold, None)
}

// Default values
fn default_object_fit() -> ObjectFit {
    ObjectFit::Cover
}

fn default_text_align() -> TextAlignType {
    TextAlignType::Left
}

fn default_line_height() -> f32 {
    1.5
}

fn default_bold() -> bool {
    false
}

fn default_padding() -> f32 {
    0.0
}

fn default_text_direction() -> TextDirectionType {
    TextDirectionType::Ltr
}

// Main poster generator struct
pub struct PosterGenerator {
    width: u32,
    height: u32,
    background_color: String,
    elements: Vec<Box<dyn PosterElement>>,
}

// Element trait
trait PosterElement {
    fn z_index(&self) -> i32;
    fn render(&self, canvas: &Canvas) -> Result<()>;
}

// Implement background element
impl PosterElement for BackgroundElement {
    fn z_index(&self) -> i32 {
        -1000 // Background always at the bottom
    }
    
    fn render(&self, canvas: &Canvas) -> Result<()> {
        // Parse color
        let color = parse_color(&self.color);
        
        // Create paint
        let mut paint = Paint::default();
        paint.set_color(color);
        paint.set_anti_alias(true);
        
        // Get canvas dimensions
        let width = canvas.base_layer_size().width;
        let height = canvas.base_layer_size().height;
        
        if let Some(radius) = &self.radius {
            // Draw with rounded corners
            let _rect = Rect::new(0.0, 0.0, width as f32, height as f32);
            let path = create_rounded_rect_path(0.0, 0.0, width as f32, height as f32, radius);
            canvas.draw_path(&path, &paint);
        } else {
            // Fill the entire canvas
            canvas.clear(color);
        }
        
        // If there's an image, draw it on top
        if let Some(img_path) = &self.image {
            if let Ok(img) = load_image(img_path) {
                // Scale image to fit
                let scaled_img = scale_image(img, width as f32, height as f32, &ObjectFit::Cover)?;
                
                // Create a mask if radius is specified
                if let Some(radius) = &self.radius {
                    let _rect = Rect::new(0.0, 0.0, width as f32, height as f32);
                    canvas.save();
                    
                    // Create clip path
                    let path = create_rounded_rect_path(0.0, 0.0, width as f32, height as f32, radius);
                    canvas.clip_path(&path, None, Some(true));
                    
                    // Draw image
                    canvas.draw_image(scaled_img, Point::new(0.0, 0.0), None);
                    
                    canvas.restore();
                } else {
                    // Draw without mask
                    canvas.draw_image(scaled_img, Point::new(0.0, 0.0), None);
                }
            }
        }
        
        Ok(())
    }
}

// Implement image element
impl PosterElement for ImageElement {
    fn z_index(&self) -> i32 {
        self.z_index.unwrap_or(0)
    }
    
    fn render(&self, canvas: &Canvas) -> Result<()> {
        // Load image
        let img = load_image(&self.src)?;
        
        // Scale image according to object_fit
        let scaled_img = scale_image(
            img,
            self.width,
            self.height,
            &self.object_fit,
        )?;
        
        // Apply radius if specified
        if let Some(radius) = &self.radius {
            canvas.save();
            
            // Create clip path
            let path = create_rounded_rect_path(
                self.x,
                self.y,
                self.width,
                self.height,
                radius,
            );
            canvas.clip_path(&path, None, Some(true));
            
            // Draw image
            canvas.draw_image(scaled_img, Point::new(self.x, self.y), None);
            
            canvas.restore();
        } else {
            // Draw without mask
            canvas.draw_image(scaled_img, Point::new(self.x, self.y), None);
        }
        
        Ok(())
    }
}

// Implement text element
impl PosterElement for TextElement {
    fn z_index(&self) -> i32 {
        self.z_index.unwrap_or(0)
    }
    
    fn render(&self, canvas: &Canvas) -> Result<()> {
        // Parse color
        let color = parse_color(&self.color);
        
        // Prepare full text content
        let full_text = match &self.prefix {
            Some(prefix) => format!("{}{}", prefix, self.text),
            None => self.text.clone(),
        };
        
        // Auto-detect text direction if not explicitly set
        let text_direction = match self.direction {
            TextDirectionType::Rtl => TextDirectionType::Rtl,
            TextDirectionType::Ltr => {
                if is_rtl_text(&full_text) {
                    TextDirectionType::Rtl
                } else {
                    TextDirectionType::Ltr
                }
            }
        };
        
        // Get appropriate font for the text with optional font family
        let font = get_font_for_text_with_family(&full_text, self.font_size, self.bold, self.font_family.as_deref());
        
        // Use TextLayout for proper RTL and complex text rendering
        self.render_with_text_layout(canvas, &full_text, &text_direction, &font, color)?;
        
        Ok(())
    }
}

impl TextElement {
    fn render_with_text_layout(&self, canvas: &Canvas, full_text: &str, text_direction: &TextDirectionType, font: &Font, color: Color) -> Result<()> {
        let mut paint = Paint::default();
        paint.set_color(color);
        paint.set_anti_alias(true);
        
        // For RTL text, we need special handling
        let processed_text = if matches!(text_direction, TextDirectionType::Rtl) {
            // For RTL languages like Uyghur, we need to process the text
            // This is a simplified approach - in a full implementation you'd want
            // proper Unicode Bidirectional Algorithm (BiDi) processing
            self.process_rtl_text(full_text)
        } else {
            full_text.to_string()
        };
        
        // Draw background if specified
        if let Some(bg_color_str) = &self.background_color {
            let bg_color = parse_color(bg_color_str);
            let mut bg_paint = Paint::default();
            bg_paint.set_color(bg_color);
            
            // Measure text to determine background size
            let (text_width, text_height) = measure_text_with_font(&processed_text, font);
            
            let bg_width = self.width.unwrap_or_else(|| text_width + self.padding * 2.0);
            let bg_height = self.height.unwrap_or_else(|| text_height + self.padding * 2.0);
            
            // Adjust x position based on text alignment
            let bg_x = match (self.align, text_direction) {
                (TextAlignType::Left, TextDirectionType::Ltr) => self.x - self.padding,
                (TextAlignType::Right, TextDirectionType::Ltr) => self.x - bg_width + self.padding,
                (TextAlignType::Center, _) => self.x - bg_width / 2.0,
                // For RTL text, reverse alignment
                (TextAlignType::Left, TextDirectionType::Rtl) => self.x - bg_width + self.padding,
                (TextAlignType::Right, TextDirectionType::Rtl) => self.x - self.padding,
            };
            
            let bg_y = self.y - text_height - self.padding;
            
            // Draw background with optional radius
            if let Some(radius) = &self.border_radius {
                let path = create_rounded_rect_path(bg_x, bg_y, bg_width, bg_height, radius);
                canvas.draw_path(&path, &bg_paint);
            } else {
                let rect = Rect::new(bg_x, bg_y, bg_x + bg_width, bg_y + bg_height);
                canvas.draw_rect(rect, &bg_paint);
            }
        }
        
        // Handle multi-line text if max_width is specified
        if let Some(max_width) = self.max_width {
            let lines = break_text_rtl(&processed_text, max_width, font, self.max_lines);
            
            for (i, line) in lines.iter().enumerate() {
                let y_pos = self.y + (i as f32 * self.font_size * self.line_height);
                draw_text_line_improved(canvas, line, self.x, y_pos, font, &paint, text_direction, &self.align);
            }
        } else {
            // Single line text
            draw_text_line_improved(canvas, &processed_text, self.x, self.y, font, &paint, text_direction, &self.align);
        }
        
        Ok(())
    }
    
    // Process RTL text for better display
    fn process_rtl_text(&self, text: &str) -> String {
        // For Arabic script text (including Uyghur), we should NOT reverse the text
        // because Skia Safe should handle the correct display direction
        // Reversing would break ligatures and proper text shaping
        
        // Instead, we preserve the original text and let Skia handle the RTL rendering
        if is_rtl_text(text) {
            // Keep original order for proper ligature rendering
            text.to_string()
        } else {
            text.to_string()
        }
    }
}

// Implementation for PosterGenerator
impl PosterGenerator {
    pub fn new(width: u32, height: u32, background_color: String) -> Self {
        Self {
            width,
            height,
            background_color,
            elements: Vec::new(),
        }
    }
    
    pub fn add_background(&mut self, background: BackgroundElement) -> &mut Self {
        self.elements.push(Box::new(background));
        self
    }
    
    pub fn add_image(&mut self, image: ImageElement) -> &mut Self {
        self.elements.push(Box::new(image));
        self
    }
    
    pub fn add_text(&mut self, text: TextElement) -> &mut Self {
        self.elements.push(Box::new(text));
        self
    }
    
    pub fn clear(&mut self) -> &mut Self {
        self.elements.clear();
        self
    }
    
    pub fn set_elements(&mut self, elements: Vec<Element>) -> &mut Self {
        self.clear();
        
        for element in elements {
            match element {
                Element::Background(bg) => self.add_background(bg),
                Element::Image(img) => self.add_image(img),
                Element::Text(txt) => self.add_text(txt),
            };
        }
        
        self
    }
    
    pub fn generate(&self) -> Result<Vec<u8>> {
        // Create surface
        let mut surface = Surface::new_raster_n32_premul((self.width as i32, self.height as i32)).ok_or_else(|| {
            PosterError::RenderError("Failed to create surface".to_string())
        })?;
        
        {
            // Get canvas
            let canvas = surface.canvas();
            
            // Fill with background color
            let bg_color = parse_color(&self.background_color);
            canvas.clear(bg_color);
            
            // Sort elements by z-index
            let mut sorted_elements = self.elements.iter().collect::<Vec<_>>();
            sorted_elements.sort_by_key(|e| e.z_index());
            
            // Render each element
            for element in sorted_elements {
                element.render(canvas)?;
            }
        }
        
        // Encode as PNG
        let image = surface.image_snapshot();
        let data = image.encode_to_data(EncodedImageFormat::PNG).ok_or_else(|| {
            PosterError::OutputError("Failed to encode image as PNG".to_string())
        })?;
        
        Ok(data.as_bytes().to_vec())
    }
    
    pub fn generate_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let png_data = self.generate()?;
        
        // Save to file
        std::fs::write(path, png_data)?;
        
        Ok(())
    }
    
    pub fn generate_base64(&self) -> Result<String> {
        let png_data = self.generate()?;
        
        // Encode to base64
        let base64 = general_purpose::STANDARD.encode(&png_data);
        
        Ok(format!("data:image/png;base64,{}", base64))
    }
}

// Utility functions
fn parse_color(color_str: &str) -> Color {
    if color_str.starts_with('#') {
        // Parse hex color
        let hex = &color_str[1..];
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return Color::from_rgb(r, g, b);
            }
        } else if hex.len() == 8 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
                u8::from_str_radix(&hex[6..8], 16),
            ) {
                return Color::from_argb(a, r, g, b);
            }
        }
    }
    
    // Default to black if parsing fails
    Color::BLACK
}

fn load_image(path: &str) -> Result<Image> {
    // Check if path is a base64 string
    if path.starts_with("data:image/") {
        let base64_data = path.split(',').nth(1).ok_or_else(|| {
            PosterError::ImageLoadError("Invalid base64 image format".to_string())
        })?;
        
        let bytes = general_purpose::STANDARD.decode(base64_data)?;
        let data = Data::new_copy(&bytes);
        
        let image = Image::from_encoded(data).ok_or_else(|| {
            PosterError::ImageLoadError("Failed to decode base64 image".to_string())
        })?;
        
        return Ok(image);
    }
    
    // Otherwise load from file
    let bytes = std::fs::read(path)?;
    let data = Data::new_copy(&bytes);
    
    let image = Image::from_encoded(data).ok_or_else(|| {
        PosterError::ImageLoadError(format!("Failed to load image from: {}", path))
    })?;
    
    Ok(image)
}

fn scale_image(img: Image, width: f32, height: f32, object_fit: &ObjectFit) -> Result<Image> {
    let src_width = img.width() as f32;
    let src_height = img.height() as f32;
    
    let mut surface = match object_fit {
        ObjectFit::Cover => {
            // Calculate scale to fill the target area while maintaining aspect ratio
            let scale_x = width / src_width;
            let scale_y = height / src_height;
            let scale = scale_x.max(scale_y);
            
            let scaled_width = (src_width * scale).ceil() as i32;
            let scaled_height = (src_height * scale).ceil() as i32;
            
            // Create a surface for the scaled image
            let mut surface = Surface::new_raster_n32_premul((width as i32, height as i32)).ok_or_else(|| {
                PosterError::RenderError("Failed to create surface for scaled image".to_string())
            })?;
            
            let canvas = surface.canvas();
            
            // Calculate position to center the scaled image
            let x = (width - scaled_width as f32) / 2.0;
            let y = (height - scaled_height as f32) / 2.0;
            
            // Draw the image scaled and centered
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.scale((scale, scale));
            canvas.draw_image(img, Point::new(x / scale, y / scale), Some(&paint));
            
            surface
        },
        ObjectFit::Contain => {
            // Calculate scale to fit within the target area while maintaining aspect ratio
            let scale_x = width / src_width;
            let scale_y = height / src_height;
            let scale = scale_x.min(scale_y);
            
            let scaled_width = (src_width * scale) as i32;
            let scaled_height = (src_height * scale) as i32;
            
            // Create a surface for the scaled image
            let mut surface = Surface::new_raster_n32_premul((width as i32, height as i32)).ok_or_else(|| {
                PosterError::RenderError("Failed to create surface for scaled image".to_string())
            })?;
            
            let canvas = surface.canvas();
            
            // Calculate position to center the scaled image
            let x = (width - scaled_width as f32) / 2.0;
            let y = (height - scaled_height as f32) / 2.0;
            
            // Draw the image scaled and centered
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            let src_rect = Rect::new(0.0, 0.0, src_width, src_height);
            let dest_rect = Rect::new(x, y, x + scaled_width as f32, y + scaled_height as f32);
            canvas.draw_image_rect(img, Some((&src_rect, skia_safe::canvas::SrcRectConstraint::Fast)), dest_rect, &paint);
            
            surface
        },
        ObjectFit::Stretch => {
            // Create a surface for the stretched image
            let mut surface = Surface::new_raster_n32_premul((width as i32, height as i32)).ok_or_else(|| {
                PosterError::RenderError("Failed to create surface for stretched image".to_string())
            })?;
            
            let canvas = surface.canvas();
            
            // Draw the image stretched to fill the target area
            let src_rect = Rect::new(0.0, 0.0, src_width, src_height);
            let dest_rect = Rect::new(0.0, 0.0, width, height);
            
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_rect(img, Some((&src_rect, skia_safe::canvas::SrcRectConstraint::Fast)), dest_rect, &paint);
            
            surface
        }
    };
    
    Ok(surface.image_snapshot())
}

fn create_rounded_rect_path(x: f32, y: f32, width: f32, height: f32, radius: &Radius) -> SkPath {
    let mut path = SkPath::new();
    
    match radius {
        Radius::Single(r) => {
            let r = r.min(width / 2.0).min(height / 2.0);
            path.add_round_rect(
                Rect::new(x, y, x + width, y + height),
                (r, r), 
                None
            );
        },
        Radius::Multiple(corners) => {
            let tl = corners[0].min(width / 2.0).min(height / 2.0);
            let tr = corners[1].min(width / 2.0).min(height / 2.0);
            let br = corners[2].min(width / 2.0).min(height / 2.0);
            let bl = corners[3].min(width / 2.0).min(height / 2.0);
            
            // Drawing a path with different corner radii
            path.move_to((x + tl, y));
            path.line_to((x + width - tr, y));
            if tr > 0.0 {
                path.quad_to((x + width, y), (x + width, y + tr));
            }
            path.line_to((x + width, y + height - br));
            if br > 0.0 {
                path.quad_to((x + width, y + height), (x + width - br, y + height));
            }
            path.line_to((x + bl, y + height));
            if bl > 0.0 {
                path.quad_to((x, y + height), (x, y + height - bl));
            }
            path.line_to((x, y + tl));
            if tl > 0.0 {
                path.quad_to((x, y), (x + tl, y));
            }
            path.close();
        }
    }
    
    path
}

// Improved text measurement with better font support
fn measure_text_with_font(text: &str, font: &Font) -> (f32, f32) {
    // Use Skia's text measurement
    let blob = TextBlob::new(text, font).unwrap_or_else(|| {
        TextBlob::new(" ", font).unwrap() // Fallback to a space if there's an issue
    });
    
    let bounds = blob.bounds();
    (bounds.width(), bounds.height())
}

fn measure_text(text: &str, font: &Font) -> (f32, f32) {
    // Use Skia's text measurement
    let blob = TextBlob::new(text, font).unwrap_or_else(|| {
        TextBlob::new(" ", font).unwrap() // Fallback to a space if there's an issue
    });
    
    let bounds = blob.bounds();
    (bounds.width(), bounds.height())
}

fn break_text(text: &str, max_width: f32, font: &Font, max_lines: Option<u32>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let (test_width, _) = measure_text(&test_line, font);
        
        if test_width <= max_width || current_line.is_empty() {
            current_line = test_line;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            
            if let Some(max) = max_lines {
                if lines.len() >= max as usize - 1 {
                    break;
                }
            }
        }
    }
    
    if !current_line.is_empty() {
        if let Some(max) = max_lines {
            if lines.len() >= max as usize {
                // Truncate last line with ellipsis
                let last_line = lines.last_mut().unwrap();
                *last_line = truncate_with_ellipsis(last_line, max_width, font);
            } else {
                lines.push(current_line);
            }
        } else {
            lines.push(current_line);
        }
    }
    
    lines
}

fn truncate_with_ellipsis(text: &str, max_width: f32, font: &Font) -> String {
    let ellipsis = "...";
    let (ellipsis_width, _) = measure_text(ellipsis, font);
    
    let (text_width, _) = measure_text(text, font);
    if text_width <= max_width {
        return text.to_string();
    }
    
    let available_width = max_width - ellipsis_width;
    let mut result = String::new();
    
    for ch in text.chars() {
        let test_text = format!("{}{}", result, ch);
        let (test_width, _) = measure_text(&test_text, font);
        
        if test_width <= available_width {
            result.push(ch);
        } else {
            break;
        }
    }
    
    format!("{}{}", result, ellipsis)
}

// RTL-aware text breaking
fn break_text_rtl(text: &str, max_width: f32, font: &Font, max_lines: Option<u32>) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    // For RTL text, we need to be careful about word boundaries
    let words: Vec<&str> = if is_rtl_text(text) {
        // For RTL languages, split by spaces but preserve character order
        text.split_whitespace().collect()
    } else {
        text.split_whitespace().collect()
    };
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let (test_width, _) = measure_text_with_font(&test_line, font);
        
        if test_width <= max_width || current_line.is_empty() {
            current_line = test_line;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            
            if let Some(max) = max_lines {
                if lines.len() >= max as usize - 1 {
                    break;
                }
            }
        }
    }
    
    if !current_line.is_empty() {
        if let Some(max) = max_lines {
            if lines.len() >= max as usize {
                // Truncate last line with ellipsis
                let last_line = lines.last_mut().unwrap();
                *last_line = truncate_with_ellipsis_rtl(last_line, max_width, font);
            } else {
                lines.push(current_line);
            }
        } else {
            lines.push(current_line);
        }
    }
    
    lines
}

fn truncate_with_ellipsis_rtl(text: &str, max_width: f32, font: &Font) -> String {
    let ellipsis = if is_rtl_text(text) { "..." } else { "..." }; // Could use RTL ellipsis: "…"
    let (ellipsis_width, _) = measure_text_with_font(ellipsis, font);
    
    let (text_width, _) = measure_text_with_font(text, font);
    if text_width <= max_width {
        return text.to_string();
    }
    
    let available_width = max_width - ellipsis_width;
    let mut result = String::new();
    
    for ch in text.chars() {
        let test_text = format!("{}{}", result, ch);
        let (test_width, _) = measure_text_with_font(&test_text, font);
        
        if test_width <= available_width {
            result.push(ch);
        } else {
            break;
        }
    }
    
    format!("{}{}", result, ellipsis)
}

// Improved text drawing with RTL support
fn draw_text_line_improved(
    canvas: &Canvas, 
    text: &str, 
    x: f32, 
    y: f32, 
    font: &Font, 
    paint: &Paint, 
    direction: &TextDirectionType,
    align: &TextAlignType
) {
    // For RTL text (Arabic/Uyghur), use Skia's textlayout for proper shaping and direction
    if matches!(direction, TextDirectionType::Rtl) && is_rtl_text(text) {
        // Create paragraph style with RTL direction
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_direction(TextDirection::RTL);
        
        // Set text alignment
        let text_align = match align {
            TextAlignType::Left => TextAlign::Left,
            TextAlignType::Right => TextAlign::Right,
            TextAlignType::Center => TextAlign::Center,
        };
        paragraph_style.set_text_align(text_align);
        
        // Create font collection with custom UKIJBasma font
        let font_mgr = FontMgr::new();
        let mut font_collection = FontCollection::new();
        
        // Load UKIJBasma font and add to font collection if available
        if let Ok(font_data) = std::fs::read("./UKIJBasma.ttf") {
            if let Some(ukij_typeface) = font_mgr.new_from_data(&font_data, None) {
                // Create a custom font provider and add the UKIJBasma font
                let mut font_provider = TypefaceFontProvider::new();
                font_provider.register_typeface(ukij_typeface.clone(), Some("UKIJBasma"));
                let font_mgr_from_provider: FontMgr = font_provider.into();
                font_collection.set_asset_font_manager(Some(font_mgr_from_provider));
            }
        }
        
        font_collection.set_default_font_manager(font_mgr, None);
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
        
        // Create text style with UKIJBasma font family
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font.size());
        text_style.set_color(paint.color());
        
        // Set font families - prioritize UKIJBasma for RTL text
        text_style.set_font_families(&["UKIJBasma", "Arial Unicode MS", "Geeza Pro"]);
        
        // Add styled text
        paragraph_builder.push_style(&text_style);
        paragraph_builder.add_text(text);
        
        // Build and layout paragraph
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(1000.0); // Wide layout for proper text measurement
        
        // Adjust Y position for baseline
        let draw_y = y - font.size();
        
        // For center alignment, adjust X position
        let draw_x = if matches!(align, TextAlignType::Center) {
            x - paragraph.max_width() / 2.0
        } else {
            x
        };
        
        // Draw the paragraph
        paragraph.paint(canvas, Point::new(draw_x, draw_y));
        
    } else {
        // For LTR text, use standard TextBlob approach
        if let Some(blob) = TextBlob::new(text, font) {
            let (text_width, _) = measure_text_with_font(text, font);
            
            let draw_x = match align {
                TextAlignType::Left => x,
                TextAlignType::Right => x - text_width,
                TextAlignType::Center => x - text_width / 2.0,
            };
            
            canvas.draw_text_blob(blob, Point::new(draw_x, y), paint);
        }
    }
}

fn draw_text_line(canvas: &Canvas, text: &str, x: f32, y: f32, font: &Font, paint: &Paint, _direction: &TextDirectionType) {
    // Create a text blob (direction handling simplified)
    if let Some(blob) = TextBlob::new(text, font) {
        // Draw text
        canvas.draw_text_blob(blob, Point::new(x, y), paint);
    }
}

// API server module
pub mod server {
    use super::*;
    use axum::{
        extract::Json,
        routing::post,
        http::StatusCode,
        response::IntoResponse,
        Router,
    };
    use serde::{Deserialize, Serialize};
    use std::net::SocketAddr;
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PosterRequest {
        pub config: PosterConfig,
        pub format: OutputFormat,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum OutputFormat {
        Base64,
        File,
    }
    
    #[derive(Debug, Serialize)]
    pub struct PosterResponse {
        pub success: bool,
        pub data: Option<String>,
        pub error: Option<String>,
    }
    
    pub async fn run_server(port: u16) -> Result<()> {
        let app = Router::new()
            .route("/generate", post(generate_poster));
            
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        println!("Listening on {}", addr);
        
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
            
        Ok(())
    }
    
    async fn generate_poster(Json(req): Json<PosterRequest>) -> impl IntoResponse {
        let result = generate_poster_internal(req).await;
        
        match result {
            Ok(response) => (StatusCode::OK, Json(response)),
            Err(e) => {
                let error_response = PosterResponse {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            }
        }
    }
    
    async fn generate_poster_internal(req: PosterRequest) -> Result<PosterResponse> {
        // Create poster generator
        let mut generator = PosterGenerator::new(
            req.config.width,
            req.config.height,
            req.config.background_color.clone(),
        );
        
        // Add elements
        for element in req.config.elements {
            match element {
                Element::Background(bg) => generator.add_background(bg),
                Element::Image(img) => generator.add_image(img),
                Element::Text(txt) => generator.add_text(txt),
            };
        }
        
        // Generate poster
        match req.format {
            OutputFormat::Base64 => {
                let base64 = generator.generate_base64()?;
                Ok(PosterResponse {
                    success: true,
                    data: Some(base64),
                    error: None,
                })
            },
            OutputFormat::File => {
                // Generate a temporary file path
                let temp_dir = std::env::temp_dir();
                let filename = format!("poster_{}.png", chrono::Utc::now().timestamp());
                let path = temp_dir.join(filename);
                
                // Generate poster file
                generator.generate_file(&path)?;
                
                Ok(PosterResponse {
                    success: true,
                    data: Some(path.to_string_lossy().to_string()),
                    error: None,
                })
            }
        }
    }
} 
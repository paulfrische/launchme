use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    RGB(u8, u8, u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub background: Color,
    pub input: Color,
    pub cursor: Color,
    pub suggestion: Color,
    pub font: Option<String>,
    pub font_size: u16,
    pub line_spacing: i32,
    pub padding: i32,
    pub width: i32,
    pub height: i32,
}

impl Config {
    pub fn load(content: String) -> Result<String, ron::error::SpannedError> {
        ron::from_str(&content)
    }

    pub fn default_content() -> String {
        ron::to_string(&Config::default()).expect("couldn't serialize config")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: Color::RGB(0, 0, 0),
            input: Color::RGB(0, 255, 255),
            cursor: Color::RGB(255, 255, 255),
            suggestion: Color::RGB(128, 128, 128),
            font: None,
            font_size: 16,
            line_spacing: 6,
            padding: 12,
            width: 1000,
            height: 700,
        }
    }
}

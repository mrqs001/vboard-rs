use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bg_color: String,
    pub bg_type: String,
    pub opacity: f64,
    pub text_color: String,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
    pub bold: bool,
    pub remember_position: bool,
    pub pos_x: i32,
    pub pos_y: i32,
    pub keyboard_layout: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bg_color: "0, 0, 0".to_string(),
            bg_type: "solid".to_string(),
            opacity: 0.90,
            text_color: "white".to_string(),
            width: 0,
            height: 0,
            font_size: 14,
            bold: false,
            remember_position: false,
            pos_x: -1,
            pos_y: -1,
            keyboard_layout: "US".to_string(),
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|path| path.join(".config")))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vboard")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("settings.conf")
    }

    pub fn load() -> Self {
        let mut config = Self::default();
        let path = Self::config_path();
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return config,
        };

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with('[')
                || trimmed.starts_with('#')
                || trimmed.starts_with(';')
            {
                continue;
            }
            let Some((key, value)) = trimmed.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim();
            match key {
                "bg_color" => config.bg_color = value.to_string(),
                "bg_type" => config.bg_type = value.to_string(),
                "opacity" => config.opacity = value.parse().unwrap_or(config.opacity),
                "text_color" => config.text_color = value.to_string(),
                "width" => config.width = value.parse().unwrap_or(config.width),
                "height" => config.height = value.parse().unwrap_or(config.height),
                "font_size" => config.font_size = value.parse().unwrap_or(config.font_size),
                "bold" => config.bold = parse_bool(value, config.bold),
                "remember_position" => {
                    config.remember_position = parse_bool(value, config.remember_position)
                }
                "pos_x" => config.pos_x = value.parse().unwrap_or(config.pos_x),
                "pos_y" => config.pos_y = value.parse().unwrap_or(config.pos_y),
                "keyboard_layout" => config.keyboard_layout = value.to_string(),
                _ => {}
            }
        }

        config.fix_inconsistencies();
        config.update_text_color();
        config
    }

    pub fn save(&self) -> anyhow::Result<()> {
        fs::create_dir_all(Self::config_dir())?;
        let content = format!(
            "[DEFAULT]\n\
             bg_color = {}\n\
             bg_type = {}\n\
             opacity = {:.2}\n\
             text_color = {}\n\
             width = {}\n\
             height = {}\n\
             font_size = {}\n\
             bold = {}\n\
             remember_position = {}\n\
             pos_x = {}\n\
             pos_y = {}\n\
             keyboard_layout = {}\n",
            self.bg_color,
            self.bg_type,
            self.opacity,
            self.text_color,
            self.width,
            self.height,
            self.font_size,
            bool_string(self.bold),
            bool_string(self.remember_position),
            self.pos_x,
            self.pos_y,
            self.keyboard_layout
        );
        fs::write(Self::config_path(), content)?;
        Ok(())
    }

    pub fn fix_inconsistencies(&mut self) {
        if self.bg_color.contains("linear-gradient") && self.bg_type == "solid" {
            self.bg_type = "gradient".to_string();
        } else if !self.bg_color.contains("linear-gradient") && self.bg_type == "gradient" {
            self.bg_type = "solid".to_string();
        }

        if self.keyboard_layout != "US" && self.keyboard_layout != "PT" {
            self.keyboard_layout = "US".to_string();
        }
    }

    pub fn update_text_color(&mut self) {
        if self.bg_type == "gradient" {
            self.text_color = "white".to_string();
            return;
        }

        let light_bg = ["255,255,255", "220,220,220", "255,255,224", "230,230,250"];
        let normalized = self.bg_color.replace(' ', "");
        if light_bg.iter().any(|color| *color == normalized) {
            self.text_color = "#1C1C1C".to_string();
        } else {
            self.text_color = "white".to_string();
        }
    }
}

fn parse_bool(value: &str, fallback: bool) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => fallback,
    }
}

fn bool_string(value: bool) -> &'static str {
    if value {
        "True"
    } else {
        "False"
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    #[test]
    fn fix_inconsistencies_corrects_invalid_layouts() {
        let mut config = AppConfig {
            keyboard_layout: "DE".to_string(),
            ..AppConfig::default()
        };

        config.fix_inconsistencies();

        assert_eq!(config.keyboard_layout, "US");
    }

    #[test]
    fn update_text_color_uses_dark_text_for_light_solid_backgrounds() {
        let mut config = AppConfig {
            bg_color: "255,255,255".to_string(),
            bg_type: "solid".to_string(),
            ..AppConfig::default()
        };

        config.update_text_color();

        assert_eq!(config.text_color, "#1C1C1C");
    }

    #[test]
    fn update_text_color_for_gradients_stays_white() {
        let mut config = AppConfig {
            bg_color: "linear-gradient(45deg, rgb(0,0,0), rgb(255,255,255))".to_string(),
            bg_type: "gradient".to_string(),
            ..AppConfig::default()
        };

        config.update_text_color();

        assert_eq!(config.text_color, "white");
    }
}

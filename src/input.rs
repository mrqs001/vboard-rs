use std::fs;
use std::path::PathBuf;

use eitype::{EiType, EiTypeConfig};

#[derive(Clone, Debug)]
pub enum EmitAction {
    Text(String),
    Special(&'static str),
}

pub struct InputBackend {
    typer: Option<EiType>,
    restore_token: Option<String>,
    token_path: PathBuf,
    layout: String,
}

impl InputBackend {
    pub fn new(layout: &str) -> Self {
        let token_path = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vboard-rs")
            .join("ei_restore_token");
        let restore_token = fs::read_to_string(&token_path)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        Self {
            typer: None,
            restore_token,
            token_path,
            layout: layout.to_ascii_lowercase(),
        }
    }

    pub fn set_layout(&mut self, layout: &str) {
        let normalized = layout.to_ascii_lowercase();
        if self.layout != normalized {
            self.layout = normalized;
            self.typer = None;
        }
    }

    pub fn emit(&mut self, action: EmitAction, modifiers: &[String]) -> Result<(), String> {
        self.ensure_connected()?;
        let typer = self
            .typer
            .as_mut()
            .ok_or_else(|| "missing EI connection".to_string())?;

        for modifier in modifiers {
            typer
                .hold_modifier(modifier)
                .map_err(|err| err.to_string())?;
        }

        let result = match action {
            EmitAction::Text(text) => typer.type_text(&text).map_err(|err| err.to_string()),
            EmitAction::Special(key) => typer.press_key(key).map_err(|err| err.to_string()),
        };

        let release_result = typer.release_modifiers().map_err(|err| err.to_string());

        result?;
        release_result?;
        Ok(())
    }

    fn ensure_connected(&mut self) -> Result<(), String> {
        if self.typer.is_some() {
            return Ok(());
        }

        let mut config = EiTypeConfig::from_env();
        config.layout = Some(self.layout.clone());

        let (typer, new_token) =
            EiType::connect_portal_with_token(config, self.restore_token.as_deref())
                .map_err(|err| format!("failed to connect via XDG portal: {err}"))?;

        if let Some(token) = new_token {
            self.restore_token = Some(token.clone());
            if let Some(parent) = self.token_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&self.token_path, token);
        }

        self.typer = Some(typer);
        Ok(())
    }
}

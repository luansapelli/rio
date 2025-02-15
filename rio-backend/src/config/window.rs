use crate::config::defaults::*;
use serde::{Deserialize, Serialize};
use sugarloaf::core::ImageProperties;

#[derive(Default, Clone, Serialize, Deserialize, Copy, Debug, PartialEq)]
pub enum WindowMode {
    Maximized,
    Fullscreen,
    // Windowed will use width and height definition
    #[default]
    Windowed,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct Window {
    #[serde(default = "default_window_width")]
    pub width: i32,
    #[serde(default = "default_window_height")]
    pub height: i32,
    #[serde(default = "WindowMode::default")]
    pub mode: WindowMode,
    #[serde(default = "default_opacity", rename = "background-opacity")]
    pub background_opacity: f32,
    #[serde(default = "default_opacity", rename = "foreground-opacity")]
    pub foreground_opacity: f32,
    #[serde(default = "bool::default")]
    pub blur: bool,
    #[serde(rename = "background-image", skip_serializing)]
    pub background_image: Option<ImageProperties>,
}

impl Default for Window {
    fn default() -> Window {
        Window {
            width: default_window_width(),
            height: default_window_height(),
            mode: WindowMode::default(),
            background_opacity: default_opacity(),
            foreground_opacity: default_opacity(),
            background_image: None,
            blur: false,
        }
    }
}

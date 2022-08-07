use std::collections::HashMap;

use derive_assets::StaticAsset;
use egui_extras::RetainedImage;

/// Icons used in the entire application

pub struct Icons {
    pub file: Vec<RetainedImage>,
    pub folder: HashMap<Folder, RetainedImage>,
    pub status: HashMap<Status, RetainedImage>,
}

impl Icons {
    pub const SIZE: egui::Vec2 = egui::Vec2::new(22., 22.);
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            file: vec![],
            folder: Folder::to_retained_img_map(),
            status: Status::to_retained_img_map(),
        }
    }
}

#[derive(StaticAsset, Hash, PartialEq, Eq, Debug)]
pub enum Folder {
    Opened,
    Closed,
}

#[derive(StaticAsset, Hash, PartialEq, Eq, Debug)]
pub enum Status {
    Checking,
    Completed,
    Failed,
    Needed,
    Paused,
}

use kit::components::indicator::Status;
use std::fs;
use titlecase::titlecase;
use walkdir::WalkDir;

use crate::state::Theme;

pub mod format_timestamp;
//pub mod notifications;
pub mod sounds;

pub fn get_available_themes() -> Vec<Theme> {
    let mut themes = vec![];

    let theme_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".uplink/")
        .join("themes");

    for file in WalkDir::new(theme_path)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            let theme = file.path().display().to_string();

            let theme_str = theme.split('/').last().unwrap();
            let pretty_theme_str = &theme_str.replace(".scss", "");
            let pretty_theme_str = titlecase(pretty_theme_str);

            let styles = fs::read_to_string(&theme).unwrap_or_default();

            let theme = Theme {
                filename: theme_str.to_owned(),
                name: pretty_theme_str.to_owned(),
                styles,
            };

            themes.push(theme);
        }
    }

    themes
}

// converts from Warp IdentityStatus to ui_kit Status
pub fn convert_status(status: &warp::multipass::identity::IdentityStatus) -> Status {
    match status {
        warp::multipass::identity::IdentityStatus::Online => Status::Online,
        warp::multipass::identity::IdentityStatus::Away => Status::Idle,
        warp::multipass::identity::IdentityStatus::Busy => Status::DoNotDisturb,
        warp::multipass::identity::IdentityStatus::Offline => Status::Offline,
    }
}

use dioxus_desktop::{tao::window::WindowId, DesktopContext};
use kit::icons::Icon;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Weak};
use uuid::Uuid;
use wry::webview::WebView;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UI {
    // stores information related to the current call
    #[serde(skip)]
    pub current_call: Option<Call>,
    // false: the media player is anchored in place
    // true: the media player can move around
    #[serde(skip)]
    pub popout_player: bool,
    #[serde(skip)]
    pub toast_notifications: HashMap<Uuid, ToastNotification>,
    pub theme: Option<Theme>,
    pub enable_overlay: bool,
    // overlays or other windows are created via DesktopContext::new_window. they are stored here so they can be closed later.
    #[serde(skip)]
    pub overlays: Vec<Weak<WebView>>,
    #[serde(default)]
    pub sidebar: bool,
}

impl Drop for UI {
    fn drop(&mut self) {
        self.clear_overlays();
    }
}

impl UI {
    fn take_popout_id(&mut self) -> Option<WindowId> {
        self.popout_player = false;
        match self.current_call.take() {
            Some(mut call) => {
                let id = call.take_window_id();
                self.current_call = Some(call);
                id
            }
            None => None,
        }
    }
    pub fn clear_popout(&mut self, desktop_context: DesktopContext) {
        if let Some(id) = self.take_popout_id() {
            desktop_context.close_window(id);
        };
    }
    pub fn set_popout(&mut self, id: WindowId) {
        self.current_call = Some(Call::new(Some(id)));
        self.popout_player = true;
    }
    pub fn clear_overlays(&mut self) {
        for overlay in &self.overlays {
            if let Some(window) = Weak::upgrade(overlay) {
                window
                    .evaluate_script("close()")
                    .expect("failed to close webview");
            }
        }
        self.overlays.clear();
    }
    pub fn remove_overlay(&mut self, id: WindowId) {
        let to_keep: Vec<Weak<WebView>> = self
            .overlays
            .iter()
            .filter(|x| match Weak::upgrade(x) {
                None => false,
                Some(window) => {
                    if window.window().id() == id {
                        window
                            .evaluate_script("close()")
                            .expect("failed to close webview");
                        false
                    } else {
                        true
                    }
                }
            })
            .cloned()
            .collect();
        self.overlays = to_keep;
    }

    pub fn toggle_muted(&mut self) {
        self.current_call = self.current_call.clone().map(|mut x| {
            x.muted = !x.muted;
            x
        });
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar = !self.sidebar;
    }

    pub fn toggle_silenced(&mut self) {
        self.current_call = self.current_call.clone().map(|mut x| {
            x.silenced = !x.silenced;
            x
        });
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Call {
    // displays the current  video stream
    // may need changing later to accommodate video streams from multiple participants
    #[serde(skip)]
    pub popout_window_id: Option<WindowId>,
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub silenced: bool,
}

impl Call {
    pub fn new(popout_window_id: Option<WindowId>) -> Self {
        Self {
            popout_window_id,
            muted: false,
            silenced: false,
        }
    }

    pub fn take_window_id(&mut self) -> Option<WindowId> {
        self.popout_window_id.take()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ToastNotification {
    pub title: String,
    pub content: String,
    #[serde(skip)]
    pub icon: Option<Icon>,
    initial_time: u32,
    remaining_time: u32,
}

impl ToastNotification {
    pub fn init(title: String, content: String, icon: Option<Icon>, timeout: u32) -> Self {
        Self {
            title,
            content,
            icon,
            initial_time: timeout,
            remaining_time: timeout,
        }
    }
    pub fn remaining_time(&self) -> u32 {
        self.remaining_time
    }

    pub fn reset_time(&mut self) {
        self.remaining_time = self.initial_time
    }

    pub fn decrement_time(&mut self) {
        if self.remaining_time > 0 {
            self.remaining_time -= 1;
        }
    }
}

#[derive(PartialEq, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme {
    pub filename: String,
    pub name: String,
    pub styles: String,
}

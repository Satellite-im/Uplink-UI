use derive_more::Display;
use dioxus::prelude::*;

use crate::icons::{Icon, IconElement};

#[derive(Eq, PartialEq, Clone, Copy, Display)]
pub enum Platform {
    // The user is using a desktop computer
    #[display(fmt = "circle")]
    Desktop,

    // The user is using a mobile device
    #[display(fmt = "mobile")]
    Mobile,

    // The user is using a television
    #[display(fmt = "tv")]
    Tv,

    // The user is using a headless device (e.g. a server)
    #[display(fmt = "headless")]
    Headless,
}

impl Platform {
    // Convert a Platform value to an Icon value
    pub fn to_icon(&self) -> Icon {
        match self {
            Platform::Desktop => Icon::Circle,
            Platform::Mobile => Icon::DevicePhoneMobile,
            Platform::Tv => Icon::Tv,
            Platform::Headless => Icon::WrenchScrewdriver,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Display)]
pub enum Status {
    // The user is currently online
    #[display(fmt = "online")]
    Online,

    // The user is currently offline
    #[display(fmt = "offline")]
    Offline,

    // The user is currently idle
    #[display(fmt = "idle")]
    Idle,

    // The user has enabled do-not-disturb mode
    #[display(fmt = "do-not-disturb")]
    DoNotDisturb,
}

#[derive(Eq, PartialEq, Props)]
pub struct Props {
    // Whether the indicator is in a loading state
    #[props(optional)]
    loading: Option<bool>,

    // The platform the user is using
    platform: Platform,

    // The user's online status
    status: Status,
}

#[allow(non_snake_case)]
pub fn Indicator(cx: Scope<Props>) -> Element {
    let icon = cx.props.platform.to_icon();
    let status = cx.props.status;

    cx.render(rsx!(div {
        class: "indicator indicator-{status}",
        IconElement {
            icon: icon,
            class: "{cx.props.platform.to_string()}"
        }
    }))
}

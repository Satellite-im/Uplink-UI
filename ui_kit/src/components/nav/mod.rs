use dioxus::prelude::*;
use uuid::Uuid;

use crate::{Icon, IconElement, elements::{button::{Button, Appearance}, tooltip::{ArrowPosition, Tooltip}}};

pub type To = &'static str;

const STYLE: &'static str = include_str!("./style.css");

#[derive(Clone, PartialEq)]
pub struct Route {
    pub to: To,
    pub icon: Icon,
    pub name: &'static str,
}

#[derive(Props)]
pub struct Props<'a> {
    #[props(optional)]
    onnavigate: Option<EventHandler<'a, To>>,
    routes: Vec<Route>,
    #[props(optional)]
    active: Option<Route>
}

/// Tells the parent the nav was interacted with.
pub fn emit(cx: &Scope<Props>, to: &To) {
    match &cx.props.onnavigate {
        Some(f) => f.call(to.to_owned()),
        None => {},
    }
}

/// Gets the appearence for a nav button based on the active route
pub fn get_appearence(active_route: &Route, route: &Route) -> Appearance {
    if active_route.to == route.to {
        Appearance::Primary
    } else {
        Appearance::Transparent
    }
}

/// Gets the active route, or returns a void one
pub fn get_active(cx: &Scope<Props>) -> Route {
    match &cx.props.active {
        Some(f) => f.to_owned(),
        None => Route {
            to: "!void",
            name: "!void",
            icon: Icon::ExclamationTriangle
        },
    }
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let active = use_state(&cx, || get_active(&cx));

    cx.render(
        rsx!(
            style { "{STYLE}" }
            div {
                class: "nav",
                cx.props.routes.iter().map(|route| {
                    let UUID = Uuid::new_v4().to_string();

                    rsx!(
                        Button {
                            key: "{UUID}",
                            icon: route.icon,
                            onpress: move |_| {
                                active.set(route.to_owned());
                                emit(&cx, &route.to)
                            },
                            tooltip: cx.render(rsx!(Tooltip {
                                arrow_position: ArrowPosition::Bottom,
                                text: route.name.into(),
                            })),
                            appearance: get_appearence(&active, &route)
                        }
                    )
                })
            }
        )
    )
}
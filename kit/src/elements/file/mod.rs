use dioxus::prelude::*;

use crate::{
    elements::input::Input,
    icons::{Icon, IconElement},
};

#[derive(Props)]
pub struct Props<'a> {
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    disabled: Option<bool>,
    #[props(optional)]
    aria_label: Option<String>,
    #[props(optional)]
    with_rename: Option<bool>,
    #[props(optional)]
    onrename: Option<EventHandler<'a, String>>,
    #[props(optional)]
    onpress: Option<EventHandler<'a>>,
    #[props(optional)]
    loading: Option<bool>,
}

pub fn get_text(cx: &Scope<Props>) -> String {
    cx.props.text.clone().unwrap_or_default()
}

pub fn get_aria_label(cx: &Scope<Props>) -> String {
    cx.props.aria_label.clone().unwrap_or_default()
}

pub fn emit(cx: &Scope<Props>, s: String) {
    if let Some(f) = cx.props.onrename.as_ref() {
        f.call(s)
    }
}

pub fn emit_press(cx: &Scope<Props>) {
    if let Some(f) = cx.props.onpress.as_ref() {
        f.call(())
    }
}

#[allow(non_snake_case)]
pub fn File<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let text = get_text(&cx);
    let aria_label = get_aria_label(&cx);
    let placeholder = text.clone();
    let with_rename = cx.props.with_rename.unwrap_or_default();
    let disabled = cx.props.disabled.unwrap_or_default();

    let loading = cx.props.loading.unwrap_or_default();

    if loading {
        cx.render(rsx!(FileSkeletal {}))
    } else {
        cx.render(rsx!(
            div {
                class: {
                    format_args!("file {}", if disabled { "disabled" } else { "" })
                },
                aria_label: "{aria_label}",
                div {
                    class: "icon",
                    onclick: move |_| emit_press(&cx),
                    IconElement {
                        icon: Icon::Document,
                    },
                },
                with_rename.then(|| rsx! (
                    Input {
                        disabled: disabled,
                        placeholder: placeholder,
                        // todo: use is_valid
                        onreturn: move |(s, _is_valid)| emit(&cx, s)
                    }
                )),
                (!with_rename).then(|| rsx! (
                    label {
                        "{text}"
                    }
                ))
            }
        ))
    }
}

#[allow(non_snake_case)]
pub fn FileSkeletal(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "file",
            div {
                class: "icon skeletal-svg",
                IconElement {
                    icon: Icon::DocumentText,
                },
            },
            div {
                class: "skeletal skeletal-bar"
            }
        }
    ))
}

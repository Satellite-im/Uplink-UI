use dioxus::{prelude::*};
use dioxus_desktop::use_eval;
use kit::{
    elements::{button::Button, input::{Options, Input}},
};
use rfd::FileDialog;
use mime::*;

use crate::{
    utils::{
        language::{get_local_text},
    },
};

#[allow(non_snake_case)]
pub fn ProfileSettings(cx: Scope) -> Element {
    let image_state = use_state(&cx, String::new);
    let banner_state = use_state(&cx, String::new);
    let edit_mode = use_state(&cx, || false);
    let username = use_state(&cx, || "username".to_owned());
    let status_message = use_state(&cx, || "status message".to_owned());
    let warning_message = use_ref(&cx, || get_local_text("settings-profile.greater-than-32"));



    let change_banner_text = get_local_text("settings-profile.change-banner");
    let change_avatar_text = get_local_text("settings-profile.change-avatar");
    let username_limited_to_32 = get_local_text("settings-profile.limited-to-32");
    let username_less_than_4 = get_local_text("settings-profile.less-than-4");
    let status_message_limited_to_128 = get_local_text("settings-profile.limited-to-128");

    let show_texts = !**edit_mode;
    let show_edit_fields = **edit_mode;

    cx.render(rsx!(
        div {
            id: "settings-profile",
            div {
                class: "profile-header",
                div { 
                    class: "profile-banner", 
                    img {
                        class: "profile-banner-photo",
                        src: "{banner_state}",
                        height: "100%",
                        width: "100%",
                        onclick: move |_| change_profile_image(banner_state),
                    },
                    p {class: "change-banner-text", "{change_banner_text}" },
            },
                div {
                    class: "profile-picture",
                    img {
                        class: "profile-avatar",
                        src: "{image_state}",
                        onclick: move |_| change_profile_image(image_state),
                    },
                    p {class: "change-avatar-text", "{change_avatar_text}" },
                }
                div {
                    class: "plus-button", 
                    Button {
                        icon: kit::icons::Icon::Plus,
                        onpress: move |_| change_profile_image(image_state),
                    },
                },
                show_texts.then(|| rsx!(
                    div {
                        class: "edit-button", 
                        Button {
                            text: get_local_text("settings-profile.edit-button"),
                            onpress: move |_| edit_mode.set(!edit_mode),
                        },
                    },
                    p { 
                        class: "username",
                        "{username}"
                    },
                    p { 
                        class: "status-message",
                        "{status_message}"
                    }
                ))
                show_edit_fields.then(|| 
                    {
                    let new_username_val = use_ref(&cx,  || format!("{}", username));
                    let new_status_message_val = use_ref(&cx, || format!("{}", status_message));
                    rsx!(
                    div {
                        class: "edit-button", 
                        Button {
                            text: get_local_text("settings-profile.save-button"),
                            onpress: move |_| {
                                let new_username = new_username_val.with(|i| i.clone());
                                if new_username_val.read().len() < 4 {
                                    *warning_message.write_silent() = get_local_text("settings-profile.less-than-4");
                                    let script = r#"
                                    document.getElementById("username_warning_2").style.display = 'block'
                                    document.getElementById("status_message_edit").style.top = "324px";
                                    "#;
                                    use_eval(cx)(script.to_owned());
                                    return;
                                }
                                if new_username.len() > 3 {
                                    username.set(new_username.clone());
                                }
                                let new_status_message = new_status_message_val.with(|i| i.clone());
                                status_message.set(new_status_message);
                                edit_mode.set(!edit_mode);
                            },
                        },
                        
                    },
                    div {
                        class: "username", 
                            Input {
                            id: "username_text_field".to_owned(),
                            focus: true,
                            placeholder: "".to_owned(),
                            default_text: format!("{}", if !new_username_val.read().is_empty() {username} else {""}),
                            max_lenght: 32,
                            disabled: false,
                            onchange: move |value| {
                                let val: String = value;
                                *new_username_val.write() = val.clone();
                                if val.len() == 32 {
                                    use_eval(cx)(get_limited_to_32_chars_script()[0].clone());
                                } else if val.len() < 32 && val.len() > 3 {
                                    use_eval(cx)(get_limited_to_32_chars_script()[1].clone());
                                }
                            }, 
                            options: Options {
                                with_clear_btn: true,
                                ..Options::default()
                            },
                        }
                    p {class: "username-len-counter", format!("{}/32", new_username_val.read().len())},
                    p {id: "username_warning", class: "username-warning", format!("{}", username_limited_to_32)},
                    p {id: "username_warning_2", class: "username-warning-2", format!("{}", username_less_than_4)},

                    },
                  div {
                        id: "status_message_edit",
                        class: "status-message-edit", 
                        Input {
                            placeholder: "".to_owned(),
                            disabled: false,
                            default_text: format!("{}", if !new_status_message_val.read().is_empty() {status_message} else {""}),
                            max_lenght: 128,
                            onchange: move |value| {
                                let val: String = value;
                                *new_status_message_val.write() = val.clone();
                                if val.len() == 128 {
                                    use_eval(cx)(get_limited_to_128_chars_script()[0].clone());
                                } else if val.len() < 128 {
                                    use_eval(cx)(get_limited_to_128_chars_script()[1].clone());
                                }
                            }, 
                            options: Options {
                                with_clear_btn: true,
                                ..Options::default()
                            }
                        }
                        p {class: "status-message-len-counter", format!("{}/128", new_status_message_val.read().len())},
                        p {id: "status_message_warning", class: "status-message-warning", format!("{}", status_message_limited_to_128)},
                    },
                    )}),
              
            },
        }
    ))
}

fn change_profile_image(image_state: &UseState<String>) {
    let path = match FileDialog::new().add_filter("image", &["jpg", "png", "jpeg", "svg"]).set_directory(".").pick_file() {
        Some(path) => path,
        None => return
    };

    let file = match std::fs::read(&path) {
        Ok(image_vec) => image_vec,
        Err(_) => vec![],
    };

    let filename = std::path::Path::new(&path)
    .file_name()
    .unwrap_or_else(|| std::ffi::OsStr::new(""))
    .to_str()
    .unwrap()
    .to_string();

    let parts_of_filename: Vec<&str> = filename.split('.').collect();

    //Since files selected are filtered to be jpg, jpeg, png or svg the last branch is not reachable
    let mime = match parts_of_filename.last() {
        Some(m) => {
            match *m {
                "png" => IMAGE_PNG.to_string(),
                "jpg" => IMAGE_JPEG.to_string(),
                "jpeg" => IMAGE_JPEG.to_string(),
                "svg" => IMAGE_SVG.to_string(),
                &_ => "".to_string(),
            }
        },
        None =>  "".to_string(),
    };

    let image = match &file.len() {
        0 => "".to_string(),
        _ => {
            let prefix = format!("data:{};base64,", mime);
            let base64_image = base64::encode(&file);
            let img = prefix + base64_image.as_str();
            img
        }
    };

    // TODO: Add upload picture to multipass here

    image_state.set(image);
}

fn get_limited_to_32_chars_script() -> Vec<String> {
    let script_forward = r#"
        const element = document.getElementById("status_message_edit");
        let top = parseInt(getComputedStyle(element).top, 10)
        if (top < 324) {
            const interval = setInterval(() => {
                top += 324 - top;
                element.style.top = `${top}px`;
                }, 1)
                
                setTimeout(() => {
                clearInterval(interval);
                }, 1)
            }
            document.getElementById("username_warning").style.display = 'block'
        "#;
    let script_back =  r#"
        document.getElementById("username_warning").style.display = 'none'
        document.getElementById("username_warning_2").style.display = 'none'
        document.getElementById("status_message_edit").style.top = "308px";
    "#;
    return vec![script_forward.to_owned(), script_back.to_owned()];
}

fn get_limited_to_128_chars_script() -> Vec<String> {
    let script_forward = r#"
        document.getElementById("status_message_warning").style.display = 'block'
    "#;
    let script_back =  r#"
        document.getElementById("status_message_warning").style.display = 'none'
    "#;
    return vec![script_forward.to_owned(), script_back.to_owned()];
}
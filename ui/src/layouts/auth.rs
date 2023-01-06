use dioxus::prelude::*;
use dioxus_router::use_router;
use futures::channel::oneshot;
use futures::StreamExt;
use kit::{
    elements::{
        button::Button,
        input::{Input, Options, Validation},
    },
    icons::Icon,
};

use crate::{
    warp_runner::commands::{MultiPassCmd, WarpCmd},
    CHAT_ROUTE, WARP_CMD_CH,
};

#[allow(non_snake_case)]
pub fn AuthLayout(cx: Scope) -> Element {
    let warp_cmd_tx = WARP_CMD_CH.0.clone();
    let router = use_router(cx);
    let username = use_state(cx, String::new);
    //let error = use_state(cx, String::new);
    let username_valid = use_state(cx, || false);
    let pin = use_state(cx, String::new);
    let pin_valid = use_state(cx, || false);

    let username_validation = Validation {
        // The input should have a maximum length of 32
        max_length: Some(32),
        // The input should have a minimum length of 4
        min_length: Some(4),
        // The input should only contain alphanumeric characters
        alpha_numeric_only: true,
        // The input should not contain any whitespace
        no_whitespace: true,
    };

    // Set up validation options for the input field
    let pin_validation = Validation {
        // The input should have a maximum length of 32
        max_length: Some(32),
        // The input should have a minimum length of 4
        min_length: Some(4),
        // The input should only contain alphanumeric characters
        alpha_numeric_only: false,
        // The input should not contain any whitespace
        no_whitespace: true,
    };

    let ch = use_coroutine(cx, |mut rx: UnboundedReceiver<(String, String)>| {
        to_owned![warp_cmd_tx, router];
        async move {
            while let Some((username, passphrase)) = rx.next().await {
                //println!("auth got input");
                let (tx, rx) = oneshot::channel::<Result<(), warp::error::Error>>();

                warp_cmd_tx
                    .send(WarpCmd::MultiPass(MultiPassCmd::CreateIdentity {
                        username,
                        passphrase,
                        rsp: tx,
                    }))
                    .expect("AuthLayout failed to send warp command");

                let res = rx.await.expect("failed to get response from warp_runner");

                //println!("got response from warp");
                match res {
                    Ok(_) => router.replace_route(CHAT_ROUTE, None, None),
                    Err(e) => {
                        eprintln!("auth failed: {}", e);
                        todo!("handle error response");
                    }
                }
            }
        }
    });

    cx.render(rsx!(
        div {
            id: "unlock-layout",
            Input {
                is_password: false,
                icon: Icon::Identification,
                disabled: false,
                placeholder: "enter username".into(), //get_local_text("auth.enter_username"),
                options: Options {
                    with_validation: Some(username_validation),
                    with_clear_btn: true,
                    ..Default::default()
                }
                onchange: |(val, is_valid)| {
                    username.set(val);
                    username_valid.set(is_valid);
                }
            },
            Input {
                is_password: true,
                icon: Icon::Key,
                disabled: false,
                placeholder: "enter pin".into(), //get_local_text("unlock.enter_pin"),
                options: Options {
                    with_validation: Some(pin_validation),
                    with_clear_btn: true,
                    ..Default::default()
                }
                onchange: |(val, is_valid)| {
                    pin.set(val);
                    pin_valid.set(is_valid);
                }
            },
            Button {
                text: "create account".into(), // get_local_text("unlock.create_account"),
                appearance: kit::elements::Appearance::Primary,
                icon: Icon::Check,
                onpress: move |_| {
                    if *pin_valid.current() && *username_valid.current() {
                        ch.send((username.current().to_string(), pin.current().to_string()))
                    } else {
                        println!("input not valid");
                    }
                }
            }
        }
    ))
}

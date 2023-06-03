#![allow(non_snake_case)]
use std::collections::HashMap;

use dioxus::prelude::*;
use log::info;
pub fn LoginForm(cx: Scope) -> Element {
    let username = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let log_in = use_state(cx, || false);
    let testing = use_state(cx, HashMap::new);
    let testing_1 = use_state(cx, || false);
    let testing_2 = use_state(cx, || false);

    let submitLogin = move |_| {
        let mut map = HashMap::new();
        map.insert("username", (*username.current()).to_owned());
        map.insert("password", (*password.current()).to_owned());
        testing.set(map.to_owned());
        testing_1.set(true);
        cx.spawn({
            let log_in = log_in.to_owned();
            async move {
                let resp = reqwest::Client::new()
                    .post("http://localhost:8000/v1/auth/login")
                    .header("Access-Control-Allow-Origin", "*")
                    .json(&map)
                    .send()
                    .await;

                match resp {
                    Ok(data) => {
                        match data.status() {
                            reqwest::StatusCode::OK => {
                                log_in.set(true);
                                info!("Nakalog-in ako")
                            }
                            reqwest::StatusCode::UNAUTHORIZED => {
                                info!("Need to grab a new token");
                            }
                            _ => {
                                info!("Uh oh! Mali yung request mo")
                            }
                        }
                    }
                    Err(_err) => {
                        ()
                    }
                }
            }
        })
    };

    let handleUsernameChange = move |e:Event<FormData>| {
        info!("{:?}", e);
        username.set(e.data.value.to_owned());
        info!("{}", username.current());
    };

    let handlePasswordChange = move |e:Event<FormData>| {
        info!("{:?}", e);
        password.set(e.data.value.to_owned());
        info!("{}", password.current());
    };
    info!("--------");
    info!("{}",log_in.current());
    info!("{:?}",testing.current());
    info!("{}",testing_1.current());
    info!("{}",testing_2.current());
    cx.render(
        rsx!(
        form { class: "login", onsubmit: submitLogin,
            prevent_default: "onsubmit",
            h1 { class: "login-title", "Login" }
            input { r#type: "text", class: "login-input", placeholder: "Username", autofocus: true, value: "{username.current()}", onchange: handleUsernameChange}
            input { r#type: "password", class: "login-input", placeholder: "Password", value: "{password.current()}", onchange: handlePasswordChange}
            input { r#type: "submit", value: "Let's Go", class: "login-button" }
        },
        button {
            onclick: move |event| testing_2.set(true),
            "click me!"
        }
    ))
}

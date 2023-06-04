#![allow(non_snake_case)]
use std::{collections::HashMap, str::FromStr};

use dioxus::prelude::*;
use log::{error, warn, info};
use gloo::storage::{LocalStorage, Storage};

use crate::LoginStatus;

pub fn LoginForm(cx: Scope) -> Element {
    let username = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let login_status_context = use_shared_state::<LoginStatus>(cx).unwrap();

    let submitLogin = move |_| {
        let mut map = HashMap::new();
        map.insert("username", (*username.current()).to_owned());
        map.insert("password", (*password.current()).to_owned());
        
        cx.spawn({
            to_owned![login_status_context];
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
                                let token = data.headers().get("token").unwrap().to_str().unwrap();
                                info!("{}", token);
                                LocalStorage::set("token", token).ok();
                                login_status_context.write().0 = true;
                                info!("Successfully logged in")
                            }
                            reqwest::StatusCode::UNAUTHORIZED => {
                                info!("Wrong Password");
                            }
                            _ => {
                                warn!("There's something wrong with your request: {data:?}")
                            }
                        }
                    }
                    Err(err) => {
                        error!("Encounterred errror: {err}")
                    }
                }
            }
        })
    };

    let handleUsernameChange = move |e:Event<FormData>| {
        username.set(e.data.value.to_owned());
        info!("{}", username.current());
    };

    let handlePasswordChange = move |e:Event<FormData>| {
        password.set(e.data.value.to_owned());
        info!("{}", password.current());
    };
    cx.render(
        rsx!(
        form { class: "login", onsubmit: submitLogin,
            prevent_default: "onsubmit",
            h1 { class: "login-title", "Login" }
            input { r#type: "text", class: "login-input", placeholder: "Username", autofocus: true, value: "{username.current()}", onchange: handleUsernameChange}
            input { r#type: "password", class: "login-input", placeholder: "Password", value: "{password.current()}", onchange: handlePasswordChange}
            input { r#type: "submit", value: "Let's Go", class: "login-button" }
        },
    ))
}

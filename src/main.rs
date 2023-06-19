#![allow(non_snake_case)]
use components::login_form::LoginForm;
use dioxus::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use log::{info, Level};

mod components;

fn main() {
    dioxus_web::launch(App);
    // dioxus_desktop::launch(App);
}
#[derive(Clone, Copy)]
struct LoginStatus(bool);

fn App(cx: Scope) -> Element {
    // States
    let pending_items = use_state(cx, || String::new());
    let done_items = use_state(cx, || String::new());
    let pending_items_count = use_state(cx, || 0);
    let done_items_count = use_state(cx, || 0);
    use_shared_state_provider(cx, || LoginStatus(false));
    let login_status_context = use_shared_state::<LoginStatus>(cx).unwrap();
    // let test_fut = use_future!(cx, || async {
    //     let client = reqwest::Client::new();
    //     client.get("http://127.0.0.1:8000/v1/item/get")
    //     .header("token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoyLCJleHAiOjE2ODUyNTk1OTB9.yYZWwOlJycv9izUGMH8Vq49V07eiAPumO6-ARQJ87Cg")
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap()
    // });
    // let testing = test_fut.value();
    // cx.render(
    //     rsx!(
    //     div { class: "App",
    //         div { class: "mainContainer",
    //             div { class: "header",
    //                 p { "complete taks: {done_items_count} {testing}" }
    //                 p { "pending tasks: {pending_items_count}" }
    //             }
    //             h1 { "Peding Items" }
    //             h1 { "Done Items" }
    //         }
    //     }
    // ))

    // cx.render(match test_fut.value() {
    //     Some(test_hehe) =>   rsx!(
    //         div { class: "App",
    //             div { class: "mainContainer",
    //                 div { class: "header",
    //                     p { "complete taks: {done_items_count} {test_hehe}" }
    //                     p { "pending tasks: {pending_items_count}" }
    //                 }
    //                 h1 { "Peding Items" }
    //                 h1 { "Done Items" }
    //             }
    //         }
    //     ),
    //     None => rsx! { div { "Loading dogs..." } },
    // })
    console_log::init_with_level(Level::Debug);
    info!("It works");
    let login = |v: String| info!("{}", v);
    
    let handle_returned_state = |response| {
        
    };

    let get_items = || {

    };

    let check_login = |_| {
        info!("Checking");
        let token = LocalStorage::get::<String>("token");

        match token {
            Ok(value) => {
                login_status_context.write().0 = true;
                get_items();
            },
            Err(err) => ()
        }

    };

    // login_status_context.write().0 = true;
    if login_status_context.read().0 {
        cx.render(rsx!(
            div { class: "App",
                div { class: "mainContainer",
                    div { class: "header",
                        p { "Complete tasks: {done_items_count}"}
                        p { "Complete tasks: {pending_items_count}"}
                    },
                    h1 {"Pending Items"}
                    h1 {"Done Items"}
                }
            }
        ))
    } else {
        cx.render(rsx!(
            div { class: "App",
                onmounted: check_login,
                div { class: "mainContainer",
                    LoginForm {} }
            }
        ))
    }

    // cx.render(rsx!(
    //     div { class: "App",
    //         div { class: "mainContainer",
    //             if login_status_context.read().0 {
    //                 div {"Hello"}
    //             } else {
    //                 div {"Hi"}
    //             }
    //         }
    //     }
    // ))
}

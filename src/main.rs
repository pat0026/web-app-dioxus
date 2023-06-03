#![allow(non_snake_case)]
use dioxus::prelude::*;
use components::login_form::LoginForm;
use log::Level;
use log::info;

mod components;

fn main() {
    dioxus_web::launch(App);
    // dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {    
    // States  
    let pending_items = use_state(cx, || String::new());
    let done_items = use_state(cx, || String::new());
    let pending_items_count = use_state(cx, || 0);
    let done_items_count = use_state(cx, || 0);
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
    cx.render(rsx!(
        div { class: "App",
            div { class: "mainContainer", LoginForm {} }
        }
    ))

}

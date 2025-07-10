use sycamore::futures::spawn_local;
use sycamore::prelude::*;
use sycamore_router::navigate;
use crate::api::auth::{login, LoginRequest, store_token};
use crate::model::AppResponse;

#[component]
pub fn LoginScreen() -> View {
    let username = create_signal(String::new());
    let password = create_signal(String::new());
    let error_msg = create_signal(None::<String>);

    let on_submit = move |_| {
        let username = username.get_clone().to_string();
        let password = password.get_clone().to_string();
        let error_msg = error_msg.clone();

        spawn_local(async move {
            let req = LoginRequest { username, password };
            match login(&req).await {
                Ok(AppResponse { code: 0, data: Some(token), .. }) => {
                    store_token(&token);
                    navigate("/");
                }
                Ok(resp) => error_msg.set(Some(resp.message)),
                Err(e) => error_msg.set(Some(e)),
            }
        });
    };

    view! {
        div(class="max-w-sm mx-auto mt-20 p-6 border rounded shadow") {
            h2(class="text-xl font-bold mb-4") { "管理员登录" }

            input(class="w-full mb-2 p-2 border rounded", placeholder="用户名", bind:value=username)
            input(class="w-full mb-2 p-2 border rounded", placeholder="密码", r#type="password", bind:value=password)

            button(class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded", on:click=on_submit) {
                "登录"
            }

            (if let Some(msg) = error_msg.get_clone() {
                view! { p(class="text-red-600 mt-2") { (msg) } }
            } else {
                view! { span {} }
            })
        }
    }
}

use sycamore::prelude::*;
use crate::component::login::LoginScreen;
use crate::api::auth::{load_token, logout};

/// 首页组件，根据是否登录展示不同内容
#[component]
pub fn HomeView() -> View {
    // 是否已登录（根据是否存在 token）
    let is_logged_in = create_signal(load_token().is_some());

    // 登出函数：清除 token 并刷新界面
    let logout = {
        move |_| {
            logout();
            is_logged_in.set(false);
        }
    };

    view! {
        div(class="min-h-screen bg-gray-100 flex flex-col items-center justify-center p-6") {
            h1(class="text-3xl font-bold mb-6") { "欢迎来到后台" }

            (if is_logged_in.get() {
                view! {
                    div(class="bg-white shadow rounded p-6 text-center") {
                        p(class="text-lg mb-4") { "您已成功登录。" }
                        button(
                            class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded",
                            on:click=logout
                        ) {
                            "登出"
                        }
                    }
                }
            } else {
                view! {
                    div(class="w-full max-w-sm") {
                        LoginScreen()
                    }
                }
            })
        }
    }
}
use sycamore::web::{console_error, console_warn};
use web_sys::Notification;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

// 一个辅助函数，用于请求权限并显示浏览器通知
pub async fn show_browser_notification(title: &str, body: &str) {
    // 1. 请求权限
    let permission_result = JsFuture::from(Notification::request_permission().unwrap()).await;

    if let Ok(permission) = permission_result {
        // 2. 检查权限是否被授予
        if permission.as_string() == Some("granted".to_string()) {
            // 3. 创建并显示通知
            let options = web_sys::NotificationOptions::new();
            options.set_body(body);
            // 你还可以在这里设置图标: options.set_icon(Some("url/to/icon.png"));

            if let Err(e) = Notification::new_with_options(title, &options) {
                // 如果创建失败（很少见），在控制台打印错误
                console_error!("Failed to create notification:{:?}", e);
            }
        } else {
            // 如果用户拒绝了权限，在控制台打印提示
            console_warn!("Notification permission was not granted.");
        }
    }
}
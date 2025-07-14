use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;

pub fn api_info() -> Spec {
    Spec {
        info: Info {
            title: "Rust Blog Web API".to_string(),
            description: Some(
                [
                    "这是一个基于 Rust 的博客系统后端 API。",
                    "",
                    "功能模块包括：",
                    "- 博客文章的创建、更新、删除与展示",
                    "- 图片上传和管理",
                    "- Markdown 渲染",
                    "- 用户认证与权限校验（JWT）",
                    "- 日志记录与访问管理",
                    "",
                    "本接口文档基于 Apistos 生成。",
                ].join("\n"),
            ),
            version: "v1.0.0".to_string(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "/".to_string(),
            description: Some("Blog Web API Root".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }
}
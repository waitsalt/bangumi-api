# Bangumi API Rust 客户端 (bangumi-api)

[![Crates.io](https://img.shields.io/crates/v/bangumi-api.svg)](https://crates.io/crates/bangumi-api)
[![Documentation](https://docs.rs/bangumi-api/badge.svg)](https://docs.rs/bangumi-api)
[![Docs.rs](https://docs.rs/bangumi-api/badge.svg)](https://docs.rs/bangumi-api)


一个现代、异步的 [Bangumi API](https://bangumi.github.io/api/) Rust 客户端库。

本项目基于 `tokio` 和 `reqwest` 构建，提供了一套完整的强类型数据模型，让您能以类型安全、符合人体工程学的方式与 Bangumi API 进行交互。

## ✨ 特性

- **完全异步**: 基于 `async/await` 和 `tokio` 运行时，提供非阻塞 I/O。
- **强类型模型**: 为所有 API 的请求和响应提供了完整的 `serde` 序列化/反序列化数据结构。
- **全面的 API 覆盖**: 封装了大部分 V0 版本的 API，包括：
    - **User**: 用户信息
    - **Subject**: 条目（动画、书籍、音乐、游戏、三次元）
    - **Character**: 角色
    - **Person**: 人物（声优、制作人等）
    - **Collection**: 收藏管理
    - **Episode**: 章节
    - **Index**: 目录
    - **Revision**: 修订历史
- **简洁的调用方式**: 统一的 `BangumiClient` 入口，链式调用构建请求。
- **统一的错误处理**: 将 Bangumi API 返回的错误信息解析为 `BangumiError` 结构体，方便调试。
- **自带测试**: 包含覆盖了大部分 API 端点的单元测试。

## 📦 安装

将此库添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
bangumi-api = "0.1.0" # 请使用 crates.io 上的最新版本
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

## 🚀 快速开始

以下示例展示了如何使用 `bangumi-api`。

### 1. 实例化客户端

对于访问公开数据，可以直接使用默认客户端。

```rust
use bangumi_api::common::model::BangumiClient;

let client = BangumiClient::default();
```

如果需要访问需要授权的接口（如收藏、查看个人信息等），您需要在初始化客户端时提供 `access_token`。

```rust
use bangumi_api::common::model::BangumiClient;

let my_token = "YOUR_ACCESS_TOKEN".to_string();

let auth_client = BangumiClient::new(
    "https://api.bgm.tv".to_string(), // 默认 API 地址
    None, // 使用默认 User-Agent
    Some(my_token),
);
```
> **提示**: 建议从环境变量或配置文件中读取 `access_token`，避免硬编码在代码中。

### 2. 调用 API

所有 API 方法都作为 `BangumiClient` 的方法提供。

```rust
use bangumi_api::common::model::BangumiClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // --- 调用公开 API (无需授权) ---
    let client = BangumiClient::default();

    // 获取条目详情 (ID: 2 -> "AIR")
    match client.get_subject(2).await {
        Ok(subject) => {
            println!("成功获取条目: {} / {}", subject.name, subject.name_cn);
            println!("简介: {}", subject.summary);
        }
        Err(e) => {
            eprintln!("获取条目失败: {}", e);
        }
    }

    // --- 调用需授权的 API ---
    let token = std::env::var("BANGUMI_ACCESS_TOKEN").ok();
    if let Some(access_token) = token {
        let auth_client = BangumiClient {
            access_token: Some(access_token),
            ..Default::default()
        };

        // 获取当前登录用户信息
        match auth_client.get_me().await {
            Ok(me) => {
                println!("\n成功获取当前用户信息, 你好, {}!", me.nickname);
            }
            Err(e) => {
                eprintln!("\n获取个人信息失败 (可能是 Token 无效或过期): {}", e);
            }
        }
    } else {
        println!("\n未设置 BANGUMI_ACCESS_TOKEN 环境变量，跳过授权接口测试。");
    }

    Ok(())
}
```

## 📚 API 模块

本库根据 Bangumi API 的功能对模块进行了划分，所有功能都通过 `BangumiClient` 的方法提供。

- **`character` (角色)**: 搜索、获取角色详情、封面、关联条目/人物，以及收藏/取消收藏角色。
- **`collection` (收藏)**: 管理用户收藏。获取、添加、更新用户的条目、章节、角色、人物收藏状态。
- **`episode` (章节)**: 获取条目的分集列表和特定分集详情。
- **`indice` (目录)**: 操作用户创建的目录。获取、创建、编辑、删除目录及目录中的条目。
- **`person` (人物)**: 搜索、获取人物详情、封面、关联条目/角色，以及收藏/取消收藏人物。
- **`revision` (修订)**: 查看条目、角色、人物等的编辑历史。
- **`subject` (条目)**: 搜索、浏览、获取条目详情、封面、关联人物/角色/条目，以及每日放送日历。
- **`user` (用户)**: 获取用户公开信息、头像以及当前登录用户（自己）的详细信息。

## 🧪 运行测试

克隆本仓库后，您可以使用 Cargo 运行内置的测试套件：

```bash
cargo test
```

> **注意**:
>
> - 大部分测试会向真实的 Bangumi API 发送请求。
> - 涉及用户收藏、修改等需要授权的测试，在未提供有效 `access_token` 的情况下会请求失败，导致测试不通过。
> - 如果需要完整测试，请修改对应 `test.rs` 文件，在 `BangumiClient` 中填入您的 `access_token`。

## 🤝 贡献

欢迎任何形式的贡献！如果您发现任何 Bug、有功能建议或希望改进代码，请随时：

1.  提交一个 [Issue](https://github.com/waitsalt/bangumi-api/issues)。
2.  Fork 本项目并提交一个 Pull Request。

## 📜 许可证

本项目采用 **MIT** 许可证。详情请见 `LICENSE` 文件。

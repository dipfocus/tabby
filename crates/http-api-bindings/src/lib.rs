mod chat;
mod completion;
mod embedding;

pub use chat::create as create_chat;
pub use completion::{build_completion_prompt, create};
pub use embedding::create as create_embedding;

/// 创建 reqwest 客户端
///
/// 该函数用于创建一个 reqwest 客户端实例，用于发送 HTTP 请求。
/// 它接受一个 API 端点作为参数，并根据端点是否为本地主机来决定是否设置代理。
///
/// 参数：
/// - `api_endpoint`：API 端点，用于指定要发送请求的目标地址。
///
/// 返回值：
/// - `reqwest::Client`：创建的 reqwest 客户端实例，用于发送 HTTP 请求。
fn create_reqwest_client(api_endpoint: &str) -> reqwest::Client {
    let builder = reqwest::Client::builder();

    let is_localhost = api_endpoint.starts_with("http://localhost")
        || api_endpoint.starts_with("http://127.0.0.1");
    let builder = if is_localhost {
        builder.no_proxy()
    } else {
        builder
    };

    builder.build().unwrap()
}

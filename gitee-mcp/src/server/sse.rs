use axum::{
    extract::{State, Query},
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use std::{convert::Infallible, sync::Arc, collections::HashMap};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use serde_json::Value;
use gitee_rs::GiteeClient;
use crate::tools::handle_rpc_request;
use uuid::Uuid;
use tower_http::cors::CorsLayer;

// 定义发送端类型
type SseSender = mpsc::Sender<Result<Event, Infallible>>;

struct AppState {
    client: GiteeClient,
    enabled_toolsets: Option<String>,
    disabled_toolsets: Option<String>,
    // 维护 session_id 到 SSE 发送端的映射
    sessions: Mutex<HashMap<String, SseSender>>,
}

pub async fn run_sse_server(
    client: GiteeClient,
    host: String,
    port: u16,
    enabled_toolsets: Option<String>,
    disabled_toolsets: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState {
        client,
        enabled_toolsets,
        disabled_toolsets,
        sessions: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/sse", get(sse_handler))
        .route("/messages", post(message_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    eprintln!("Gitee MCP Server (SSE) running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let session_id = Uuid::new_v4().to_string();
    let (tx, rx) = mpsc::channel(100);

    // 根据 MCP 规范，建立连接后发送 endpoint 事件告知客户端往哪里 POST
    let endpoint_url = format!("/messages?session_id={}", session_id);
    let _ = tx.send(Ok(Event::default().event("endpoint").data(endpoint_url))).await;

    // 保存发送端
    state.sessions.lock().await.insert(session_id.clone(), tx);

    let stream = ReceiverStream::new(rx);
    
    // 我们在这里不写复杂的清理逻辑，简单处理
    Sse::new(stream)
}

async fn message_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    Json(input): Json<Value>,
) -> axum::http::StatusCode {
    let session_id = match params.get("session_id") {
        Some(id) => id,
        None => return axum::http::StatusCode::BAD_REQUEST,
    };

    // 处理请求
    let response = handle_rpc_request(
        &state.client,
        input,
        &state.enabled_toolsets,
        &state.disabled_toolsets,
    ).await;

    // Don't send response for notifications
    if response.get("ignore").is_some() {
        return axum::http::StatusCode::NO_CONTENT;
    }

    // 获取对应的 SSE 发送端
    let sessions = state.sessions.lock().await;
    if let Some(tx) = sessions.get(session_id) {
        // 将响应转换为字符串推送到 SSE 流中
        if let Ok(data) = serde_json::to_string(&response) {
            let _ = tx.send(Ok(Event::default().event("message").data(data))).await;
        }
        axum::http::StatusCode::ACCEPTED
    } else {
        axum::http::StatusCode::NOT_FOUND
    }
}

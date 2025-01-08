use axum::{
    routing::get,
    Router,
    extract::{Path, Query},
};
use serde::Deserialize;

#[tokio::main]
async fn main() {

    #[derive(Deserialize)]
    struct Params {
        id: Option<u8>,
    }

    // ハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }

    async fn item_handler() -> String {
        "Hello Item".to_string()
    }

    async fn item_id_handler(Path(id): Path<String>) -> String {
        format!("item id is {}", id)
    }

    async fn get_query_handler(Query(params): Query<Params>) -> String {
        let id = params.id.unwrap_or(0);
        format!("id is {}", id)
    }

    // ルートを定義
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/item", get(item_handler))
        .route("/item/:id", get(item_id_handler))
        .route("/query", get(get_query_handler));

    // 指定したポートにサーバを開く
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await.
        unwrap();
}
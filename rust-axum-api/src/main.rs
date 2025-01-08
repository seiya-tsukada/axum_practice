use axum::{
    routing::get,
    Router,
    extract::Path
};

#[tokio::main]
async fn main() {
    // ハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }

    async fn item_handler() -> String {
        "Hello Item".to_string()
    }

    async fn item_id_handler(Path(id): Path<String>) -> String {
        format!("item id:{}", id)
    }

    // ルートを定義
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/item", get(item_handler))
        .route("/item/:id", get(item_id_handler));

    // 指定したポートにサーバを開く
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await.
        unwrap();
}
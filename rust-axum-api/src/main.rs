use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Hello Worldと返すハンドラーを定義
    async fn root_handler() -> String {
        "Hello World".to_string()
    }

    async fn item_handler() -> String {
        "Hello Item".to_string()
    }

    // ルートを定義
    // "/"を踏むと、上で定義したroot_handlerを実行する
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/item", get(item_handler));

    // 指定したポートにサーバを開く
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
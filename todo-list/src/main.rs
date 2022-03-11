use actix_web::{get, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use thiserror::Error;

/// Todo リストのエントリー
struct TodoEntry {
    id: u32,
    text: String,
}

/// `index.html` に渡すデータ
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

/// カスタムエラー
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

/// Todo リストを表示するハンドラ
#[get("/")]
async fn index() -> Result<impl Responder, MyError> {
    let entries = vec![
        TodoEntry {
            id: 1,
            text: "First entry".to_string(),
        },
        TodoEntry {
            id: 2,
            text: "Second entry".to_string(),
        },
    ];

    // レンダリング
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}

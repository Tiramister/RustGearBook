use actix_web::{
    get, http::header, post, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
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
    Askama(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPool(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLite(#[from] rusqlite::Error),
}
impl ResponseError for MyError {}

/// Todoリストを表示するハンドラ
#[get("/")]
async fn index(pool: web::Data<Pool<SqliteConnectionManager>>) -> Result<impl Responder, MyError> {
    let conn = pool.get()?;

    // データを取得
    let mut stmt = conn.prepare("SELECT id, text FROM todo")?;
    let rows = stmt.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    // レンダリング
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

/// addリクエストのリクエストボディ
#[derive(Deserialize)]
struct AddParams {
    text: String,
}

/// ポストを追加するハンドラ
#[post("/add")]
async fn add(
    params: web::Form<AddParams>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<impl Responder, MyError> {
    // データベースへ追加
    let conn = pool.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;

    // indexへリダイレクト
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

/// deleteリクエストのリクエストボディ
#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

/// ポストを削除するハンドラ
#[post("/delete")]
async fn delete(
    params: web::Form<DeleteParams>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<impl Responder, MyError> {
    // データベースから削除
    let conn = pool.get()?;
    conn.execute("DELETE FROM todo WHERE id=?", &[&params.id])?;

    // indexへリダイレクト
    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // データベースへのコネクションプールを作成
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");

    // テーブルを作成
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL
            )",
        params![],
    )
    .expect("Failed to create a table `todo`.");

    // 各ハンドラがこのコネクションプールを使えるように、アプリケーションに渡す
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add)
            .service(delete)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}

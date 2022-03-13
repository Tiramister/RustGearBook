use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
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

/// Todo リストを表示するハンドラ
#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<impl Responder, MyError> {
    let conn = db.get()?;

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

    // アプリケーションにコネクションを渡す
    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}

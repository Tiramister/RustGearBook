use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

// actix_web::ResponseError を実装した型を自作する
#[derive(Error, Debug)]
enum MyError {}
impl ResponseError for MyError {}

/// `Hello world!` を返すハンドラ
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // App を元にサーバを生成、ポート 8080 にバインドして実行
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

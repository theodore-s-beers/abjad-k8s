#![warn(clippy::pedantic, clippy::nursery)]

use abjad::{Abjad, AbjadPrefs};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    input: String,
}

#[get("/")]
async fn index(params: web::Query<Params>) -> impl Responder {
    let abjad = params.input.abjad(AbjadPrefs::default());
    HttpResponse::Ok().body(abjad.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

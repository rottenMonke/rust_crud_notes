mod models;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use models::label::Label;
use models::note::Note;
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api")
                // ...so this handles requests for `GET /app/index.html`
                .service(show_players)
                .service(show_player_by_id)
                .service(notes_by_id),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/players")]
async fn show_players() -> HttpResponse {
    HttpResponse::Ok().body("Show players")
}

#[get("/players/{id}/{comment}")]
async fn show_player_by_id(path: web::Path<(u32, String)>) -> HttpResponse {
    let decons = path.into_inner();
    let a = decons.0;
    let b = decons.1;
    println!("{} {}", a, b);
    HttpResponse::Ok().body(format!("Player detail: {}, comment - {}", a, b))
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/notes/{id}")]
async fn notes_by_id(id: web::Path<String>) -> Result<impl Responder> {
    let obj = Note {
        id: id.to_string(),
        text: "String".to_string(),
        labels: vec![Label {
            id: "String".to_string(),
            text: "String".to_string(),
            color: "String".to_string(),
        }],
        title: "String".to_string(),
        comment: "String".to_string(),
        creation_date: "String".to_string(),
        remind_me_about_it: vec!["String".to_string()],
        author: "String".to_string(),
        group: "String".to_string(),
    };
    Ok(web::Json(obj))
}

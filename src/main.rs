use actix_web::{
    get, post,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use regex::Regex;
use renderer::render_template;
use serde::{Deserialize, Serialize};
use tera::Context;

mod renderer;
mod shortener;

#[derive(Deserialize, Serialize)]
struct FormData {
    url: String,
}

#[get("/{slug}")]
async fn retrieve_url(path: web::Path<(String,)>) -> impl Responder {
    let mut slug = path.0.clone();
    let expanded_url = shortener::instance().expand(&mut slug);

    Redirect::to(expanded_url)
}

#[post("/shorten-url")]
async fn shorten_url(form: web::Form<FormData>) -> impl Responder {
    let valid_url =
        Regex::new(r"^(https?:\/\/)([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(:\d+)?(\/[^\s]*)?$").unwrap();
    let mut context = Context::new();

    match valid_url.is_match(&form.url) {
        true => {
            let shortened_url = shortener::instance().shorten(&form.url);
            context.insert("url", &shortened_url);
            let html = render_template("response/shortened_url.html", &context);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        false => {
            context.insert("error", "Invalid url. Make sure you typed a valid url");
            let html = render_template("error", &context);
            HttpResponse::BadRequest().body(html)
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let context = Context::new();
    let html = renderer::render_template("index.html", &context);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(shorten_url)
            .service(retrieve_url)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

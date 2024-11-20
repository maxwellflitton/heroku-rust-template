//! The main entry point for the actix server
use actix_web::{
    web, App, HttpServer, Responder, HttpResponse, HttpRequest
};
use actix_cors::Cors;

#[cfg(feature = "frontend")]
use rust_embed::RustEmbed;

use data_access_layer::migrations::run_migrations;
use std::path::Path;

mod api;
use api::views_factory;


#[cfg(feature = "frontend")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html")
                      .body(include_str!("../index.html"))
}


#[cfg(feature = "frontend")]
#[derive(RustEmbed)]
#[folder = "../../../frontend/dist"]
struct FrontendAssets;


#[cfg(feature = "frontend")]
fn serve_frontend_asset(path: String) -> HttpResponse {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return HttpResponse::BadRequest()
                                    .body("404 Not Found")
    };
    match FrontendAssets::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(&file)
            .first_or_octet_stream().as_ref())
            .append_header(
                ("Cache-Control", "public, max-age=604800")
            )
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found")
    }
}


#[cfg(feature = "frontend")]
async fn catch_all(req: HttpRequest) -> impl Responder {
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish()
    }
    if req.path().contains("frontend/public") {
        return serve_frontend_asset(req.path().to_string())
    }
    let file_type = match mime_guess::from_path(&req.path())
                                 .first_raw() {
        Some(file_type) => file_type,
        None => "text/html"
    };
    if !file_type.contains("text/html") {
        return serve_frontend_asset(req.path().to_string())
    }    
    index().await
}


#[cfg(not(feature = "frontend"))]
async fn catch_all(req: HttpRequest) -> impl Responder {
    return HttpResponse::NotFound().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    run_migrations().await;

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
                                  .allow_any_method()
                                  .allow_any_header();
        App::new()
            .configure(views_factory)
            .wrap(cors)
            .default_service(web::route()
            .to(catch_all))
    })
        .bind("0.0.0.0:80")?
        .run()
        .await
}



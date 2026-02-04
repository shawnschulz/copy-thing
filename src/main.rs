use actix_web::{web, web::Bytes, App, HttpRequest, Result, HttpServer, Responder, HttpResponse, web::Html};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::fs::{File, read};
use std::io::Write;


#[derive(Serialize, Deserialize)]
struct TextData {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct imageData {
    text: String,
}

async fn home(_req: HttpRequest) -> Result<NamedFile> {
    let path : PathBuf = "index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn get_text(data: web::Data<Mutex<String>>) -> impl Responder {
    let text = data.lock().unwrap().clone();
    HttpResponse::Ok().json(TextData { text })
}

async fn get_image() -> impl Responder {
    let image_data = read("image.jpg").unwrap_or_else(|_| Vec::new());
    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(image_data)
}

async fn post_text(data: web::Data<Mutex<String>>, json: web::Json<TextData>) -> impl Responder {
    let mut text = data.lock().unwrap();
    *text = json.text.clone();
    HttpResponse::Ok().json(TextData { text: text.clone() })
}

async fn post_image(body: web::Bytes) -> impl Responder {
    println!("posting image");
    let mut file = File::create("image.jpg").unwrap();
    file.write_all(&body).unwrap();
    HttpResponse::Ok().json(TextData { text: "Image uploaded".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://0.0.0.0:3790");
    // Use a mutex so we can edit and access string data across threads
    let shared_text = web::Data::new(Mutex::new(String::from("Starter text")));

    HttpServer::new(move || App::new()
                    .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
                    .app_data(shared_text.clone())
                    .route("/", web::get().to(home))
                    .route("/api/post_text", web::post().to(post_text))
                    .route("/api/post_image", web::post().to(post_image))
                    .route("/api/get_text", web::get().to(get_text))
                    .route("/image.jpg", web::get().to(get_image))
    )
        .bind(("0.0.0.0", 3790))?
        .run()
        .await
}

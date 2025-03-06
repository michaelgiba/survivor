use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files as fs;
use std::process::Command;
use std::io;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../static/index.html"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting web server on http://localhost:8080");
    println!("Opening browser...");
    
    // Open the browser (works on most platforms)
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(&["/C", "start http://localhost:8080"]).spawn().ok();
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg("http://localhost:8080").spawn().ok();
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg("http://localhost:8080").spawn().ok();
    }
    
    // Determine the path based on build mode
    let wasm_path = if cfg!(debug_assertions) {
        "./target/wasm32-unknown-unknown/debug"
    } else {
        "./target/wasm32-unknown-unknown/release"
    };
    
    // Start the web server
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // Serve WASM files from the appropriate directory based on build mode
            .service(fs::Files::new("/wasm", wasm_path).show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
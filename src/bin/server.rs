use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use libloading::{Library, Symbol};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

type Func = unsafe fn(String) -> Result<String, String>;

#[post("/upload")]
async fn handle_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("Couldn't read the filename")
            })?
            .to_string();

        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
        let mut f = File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[get("/invoke/{function_name}/{function_args}")]
async fn handle_invoke(path: web::Path<(String, String)>) -> impl Responder {
    let (function_name, function_args) = path.into_inner();

    println!("Invoking {} with args {}", function_name, function_args);

    // Construct the path to the uploaded library
    // In Mac OS, the extension is .dylib unlike Linux which is .so
    let lib_path = format!("uploads/{}.dylib", function_name);
    println!("Loading {}", lib_path);

    if !Path::new(&lib_path).exists() {
        return HttpResponse::NotFound().body("Library not found");
    }

    let result = unsafe {
        let lib = Library::new(&lib_path).expect("Failed to load library");

        let func: Symbol<Func> = lib.get(b"invoke").expect("Failed to get symbol");

        match func(function_args) {
            Ok(result) => result,
            Err(e) => return HttpResponse::InternalServerError().body(e),
        }
    };

    HttpResponse::Ok().body(format!("Function returned: {}", result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handle_upload).service(handle_invoke))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

mod email;
mod pages;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use actix_multipart::Multipart;
use futures_util::{future, TryStreamExt};
use dotenv::dotenv;

async fn collect_photos(mut payload: Multipart) -> Result<Vec<(String, Vec<u8>)>, Error> {
    let mut files = Vec::new();
    while let Some(field) = payload.try_next().await? {
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            .unwrap_or_else(|| "file".to_string());

        let mut field_data = Vec::new();
        let mut field_stream = field;
        while let Some(chunk) = field_stream.try_next().await? {
            // Use `.as_ref()` or `[..]`
            field_data.extend_from_slice(chunk.as_ref());
        }

        files.push((filename, field_data));
    }
    Ok(files)
}

async fn upload_endpoint(payload: Multipart, email: String) -> Result<HttpResponse, Error> {
    let files = collect_photos(payload).await?;
    let result_string = email::send(files, email).await;
    return Ok(HttpResponse::Ok().content_type("text/html").body(result_string))
}

async fn upload_html() -> HttpResponse {
    // Render the HTML template
    let upload = pages::upload();
    return HttpResponse::Ok().content_type("text/html").body(upload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(path) => println!("Loaded .env from: {}", path.display()),
        Err(e) => eprintln!("Warning: Could not load .env file: {e}"),
    }


    // First server runs on port 8080, emailing user1@gmail.com
    let server1 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(upload_html))
            .route(
                "/upload",
                web::post().to(|payload: Multipart| {
                    upload_endpoint(payload, "lairdandrew11@gmail.com".to_string())
                }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Second server runs on port 8081, emailing user2@gmail.com
    let server2 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(upload_html))
            .route(
                "/upload",
                web::post().to(|payload: Multipart| {
                    upload_endpoint(payload, "second+lairdandrew11@gmail.com".to_string())
                }),
            )
    })
    .bind("127.0.0.1:8081")?
    .run();

    // Run both servers in parallel. If either fails, the whole app fails.
    future::try_join(server1, server2).await?;
    Ok(())
}


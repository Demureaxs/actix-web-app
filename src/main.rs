#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

mod config;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true
                } else {
                    passed = false
                }
                print!("{:?}", req);
                let end_result = match passed {
                    true => {
                        Either::Left(srv.call(req))
                    }, false => {
                        let resp = HttpResponse::NotImplemented().body(format!("Only {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors);
        println!("HTTP Server firing");
        return app;
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

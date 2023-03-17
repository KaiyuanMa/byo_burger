use actix_cors::Cors;
use actix_web::{
    get, middleware::Logger, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use dotenvy::dotenv;
use env_logger::Env;

mod burger;
use burger::Burger;

#[get("/ingredients")]
async fn list_ingredients() -> impl Responder {
    HttpResponse::Ok().json(Burger::list_ingredients())
}

#[post("/burgers")]
async fn create_burger(params: web::Json<Burger>) -> impl Responder {
    HttpResponse::Created().json(params.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ]),)
            .service(status_handler)
            .service(list_ingredients)
            .service(create_burger)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/status")]
async fn status_handler(_req: HttpRequest) -> impl Responder {
    HttpResponse::NoContent()
}

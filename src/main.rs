use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use self::models::*;
use diesel::prelude::*;
use actix_app::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    use self::schema::temp_emps::dsl::*;

    let connection = &mut establish_connection();
    let emp = temp_emps.limit(1).select(Post::as_select()).load(connection).expect("err");
    let name = emp[0].e_name.clone();

    HttpResponse::Ok().body(name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn gi(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[get("/template")]
async fn template() -> impl Responder {
    let template = IndexTemplate {
        name: String::from("subash"),
    };
    let value = template.render().unwrap();
    HttpResponse::Ok().body(value)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(gi)
            .service(template)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

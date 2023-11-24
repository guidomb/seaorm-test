mod models;

use std::env;

use actix_web::{error::ErrorInternalServerError, get, web, App, HttpResponse, HttpServer};
use models::posts::Entity as Post;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub type HttpResult = Result<HttpResponse, actix_web::Error>;

#[get("/authors/{author_id}/posts")]
async fn posts(author_id: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> HttpResult {
    let author_id = author_id.into_inner();
    let db = db.into_inner();
    let posts = Post::find()
        .filter(models::posts::Column::AuthorId.eq(author_id))
        .all(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&posts))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .map_err(std::io::Error::other)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(posts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

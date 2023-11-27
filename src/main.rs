use actix_web::{
    error::ErrorInternalServerError, get, guard, web, web::Data, App, HttpResponse, HttpServer,
    Result,
};
use async_graphql::{
    dataloader::DataLoader,
    dynamic::*,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use entities::prelude::*;
use lazy_static::lazy_static;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use seaorm_test::*;
use std::env;
use uuid::Uuid;

pub type HttpResult = Result<HttpResponse, actix_web::Error>;

lazy_static! {
    static ref PORT: String = env::var("PORT").unwrap_or("8000".into());
    static ref HOST: String = env::var("HOST").unwrap_or("0.0.0.0".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
    static ref URL: String = format!("http://{}:{}", HOST.as_str(), PORT.as_str());
}

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(&format!(
            "{}/graphql",
            URL.as_str()
        )))))
}

#[get("/authors/{author_id}/posts")]
async fn posts(author_id: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> HttpResult {
    let author_id = author_id.into_inner();
    let db = db.into_inner();
    let posts = Post::find()
        .filter(entities::post::Column::AuthorId.eq(author_id))
        .all(db.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let database = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let schema = seaorm_test::query_root::schema(
        database.clone(),
        orm_dataloader,
        *DEPTH_LIMIT,
        *COMPLEXITY_LIMIT,
    )
    .unwrap();

    println!("Visit GraphQL Playground at {}", *URL);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(database.clone()))
            .service(posts)
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(format!("{}:{}", HOST.as_str(), PORT.as_str()))?
    .run()
    .await
}

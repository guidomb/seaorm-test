use crate::OrmDataloader;
use async_graphql::{dataloader::DataLoader, dynamic::*};
use entities::{author, post, AuthorRole};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let mut context = BuilderContext::default();
        context.entity_input.insert_skips.push("Author.id".into());
        context.entity_input.insert_skips.push("Author.role".into());
        context.entity_input.insert_skips.push("Author.createdAt".into());
        context.entity_input.update_skips.push("Author.id".into());
        context.entity_input.update_skips.push("Author.role".into());
        context.entity_input.update_skips.push("Author.createdAt".into());
        context.entity_input.insert_skips.push("Post.id".into());
        context.entity_input.insert_skips.push("Post.createdAt".into());
        context.entity_input.update_skips.push("Post.id".into());
        context.entity_input.update_skips.push("Post.createdAt".into());
        context
    };
}

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [author, post,]);
    builder.register_enumeration::<AuthorRole>();
    let schema = builder.schema_builder();
    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };
    schema.data(database).data(orm_dataloader).finish()
}

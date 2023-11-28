use entities::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{EntityTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Author)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(author::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(author::Column::Email)
                            .string_len(254)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(author::Column::FirstName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(author::Column::LastName)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(author::Column::Username)
                            .string_len(50)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(post::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Post)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(post::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(post::Column::Title)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(post::Column::Description)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(ColumnDef::new(post::Column::Content).text().not_null())
                    .col(
                        ColumnDef::new(post::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT now()")
                            .not_null(),
                    )
                    .col(ColumnDef::new(post::Column::AuthorId).uuid().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("post_author")
                            .from(Post, post::Column::AuthorId)
                            .to(Author, author::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seeding tables
        let result = Author::insert(author::ActiveModel {
            email: Set("guido@cedalio.com".into()),
            first_name: Set("Guido".into()),
            last_name: Set("Marucci Blas".into()),
            username: Set("guidomb".into()),
            ..Default::default()
        })
        .exec(manager.get_connection())
        .await?;
        let author_01_id = result.last_insert_id;

        let result = Author::insert(author::ActiveModel {
            email: Set("nico@cedalio.com".into()),
            first_name: Set("Nicolás".into()),
            last_name: Set("Magni".into()),
            username: Set("nicomagni".into()),
            ..Default::default()
        })
        .exec(manager.get_connection())
        .await?;
        let author_02_id = result.last_insert_id;

        let posts = vec![
            post::ActiveModel {
                title: Set("Some example post".into()),
                description: Set("This is an example post used for debugging".into()),
                content: Set("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into()),
                author_id: Set(author_01_id),
                ..Default::default()
            },
            post::ActiveModel {
                title: Set("Another example post".into()),
                description: Set("This is another example post used for debugging".into()),
                content: Set("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into()),
                author_id: Set(author_02_id),
                ..Default::default()
            }
        ];
        Post::insert_many(posts)
            .exec(manager.get_connection())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Author).to_owned())
            .await?;

        Ok(())
    }
}

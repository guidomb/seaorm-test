use entities::*;
use sea_orm_migration::{prelude::*, sea_orm::ActiveEnum, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AuthorRole::name())
                    .values(AuthorRole::iden_values())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Author)
                    .add_column(
                        ColumnDef::new(author::Column::Role)
                            .custom(AuthorRole::name())
                            .not_null()
                            .default(AuthorRole::Writter),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .drop_column(author::Column::Role)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(Type::drop().name(AuthorRole::name()).to_owned())
            .await?;

        Ok(())
    }
}

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(
    EnumIter, DeriveActiveEnum, PartialEq, Eq, Clone, Copy, Debug, Serialize, async_graphql::Enum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "author_role")]
pub enum AuthorRole {
    #[sea_orm(string_value = "Writter")]
    Writter,
    #[sea_orm(string_value = "Publisher")]
    Publisher,
    #[sea_orm(string_value = "Editor")]
    Editor,
}

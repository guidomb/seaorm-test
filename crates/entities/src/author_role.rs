use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(
    EnumIter, DeriveActiveEnum, PartialEq, Eq, Clone, Copy, Debug, Serialize, async_graphql::Enum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "author_role")]
#[serde(rename_all = "UPPERCASE")]
pub enum AuthorRole {
    #[sea_orm(string_value = "WRITTER")]
    Writter,
    #[sea_orm(string_value = "PUBLISHER")]
    Publisher,
    #[sea_orm(string_value = "EDITOR")]
    Editor,
}

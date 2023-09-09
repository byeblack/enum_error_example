//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)", enum_name = "status")]
pub enum Status {
    // #[sea_orm(string_value = "Open")]
    // #[sea_orm(string_value = "열려 있는")]

    // The following situations
    // proc-macro derive panicked message: `""` is not a valid identifier
    #[sea_orm(string_value = "打开")]
    // #[sea_orm(string_value = "開ける")]
    // #[sea_orm(string_value = "Нээлттэй")]
    // #[sea_orm(string_value = "Открыть")]
    // #[sea_orm(string_value = "يفتح")]
    Open,
    #[sea_orm(string_value = "Close")]
    Close,
}

impl Default for Status {
    fn default() -> Self {
        Self::Close
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub text: Status,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

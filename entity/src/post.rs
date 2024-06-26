//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Open")]
    Open,
    #[sea_orm(string_value = "열려_있는")]
    열려있는,
    #[sea_orm(string_value = "打开")]
    打开,
    #[sea_orm(string_value = "開ける")]
    開ける,
    #[sea_orm(string_value = "Нээлттэй")]
    Нээлттэй,
    #[sea_orm(string_value = "Открыть")]
    Открыть,
    #[sea_orm(string_value = "يفتح")]
    يفتح,
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

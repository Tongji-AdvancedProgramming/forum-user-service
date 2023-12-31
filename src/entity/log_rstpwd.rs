//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "log_rstpwd")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub log_rstpwd_id: i32,
    pub log_rstpwd_no: String,
    pub log_rstpwd_opno: String,
    pub log_rstpwd_ipaddr: Option<String>,
    pub log_rstpwd_date: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::LogRstpwdNo",
        to = "super::student::Column::StuNo",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Student2,
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::LogRstpwdOpno",
        to = "super::student::Column::StuNo",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Student1,
}

impl ActiveModelBehavior for ActiveModel {}

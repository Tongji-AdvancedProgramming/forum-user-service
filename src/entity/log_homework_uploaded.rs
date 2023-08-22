//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "log_homework_uploaded")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub log_hwup_id: i32,
    pub log_hwup_opno: String,
    pub log_hwup_ipaddr: Option<String>,
    pub log_hwup_date: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub log_hwup_comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::LogHwupOpno",
        to = "super::student::Column::StuNo",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Student,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

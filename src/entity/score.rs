//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "score")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub stu_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub course_id: String,
    pub score: Option<i32>,
    pub record_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::course::Entity",
        from = "Column::CourseId",
        to = "super::course::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Course,
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StuId",
        to = "super::student::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Student,
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

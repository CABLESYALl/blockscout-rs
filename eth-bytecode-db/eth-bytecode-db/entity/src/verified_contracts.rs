//! SeaORM Entity. Generated by sea-orm-codegen 0.10.1

use super::sea_orm_active_enums::{BytecodeType, VerificationType};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "verified_contracts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub source_id: i64,
    pub raw_bytecode: Vec<u8>,
    pub bytecode_type: BytecodeType,
    pub verification_settings: Json,
    pub verification_type: VerificationType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sources::Entity",
        from = "Column::SourceId",
        to = "super::sources::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Sources,
}

impl Related<super::sources::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sources.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
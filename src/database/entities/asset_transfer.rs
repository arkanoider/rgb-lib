//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "asset_transfer"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub idx: i32,
    pub user_driven: bool,
    pub batch_transfer_idx: i32,
    pub asset_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    UserDriven,
    BatchTransferIdx,
    AssetId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Asset,
    BatchTransfer,
    Coloring,
    Transfer,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::Integer.def(),
            Self::UserDriven => ColumnType::Boolean.def(),
            Self::BatchTransferIdx => ColumnType::Integer.def(),
            Self::AssetId => ColumnType::String(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Asset => Entity::belongs_to(super::asset::Entity)
                .from(Column::AssetId)
                .to(super::asset::Column::AssetId)
                .into(),
            Self::BatchTransfer => Entity::belongs_to(super::batch_transfer::Entity)
                .from(Column::BatchTransferIdx)
                .to(super::batch_transfer::Column::Idx)
                .into(),
            Self::Coloring => Entity::has_many(super::coloring::Entity).into(),
            Self::Transfer => Entity::has_many(super::transfer::Entity).into(),
        }
    }
}

impl Related<super::asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl Related<super::batch_transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BatchTransfer.def()
    }
}

impl Related<super::coloring::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Coloring.def()
    }
}

impl Related<super::transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transfer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

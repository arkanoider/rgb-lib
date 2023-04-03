//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

use crate::database::ConsignmentTransport;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "consignment_endpoint"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub idx: i64,
    pub protocol: ConsignmentTransport,
    pub endpoint: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    Protocol,
    Endpoint,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TransferConsignmentEndpoint,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::BigInteger.def(),
            Self::Protocol => ColumnType::SmallInteger.def(),
            Self::Endpoint => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransferConsignmentEndpoint => {
                Entity::has_many(super::transfer_consignment_endpoint::Entity).into()
            }
        }
    }
}

impl Related<super::transfer_consignment_endpoint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferConsignmentEndpoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(TokenOnLedger::Table)
					.col(ColumnDef::new(TokenOnLedger::ChainId).string().not_null())
					.col(ColumnDef::new(TokenOnLedger::TokenId).string().not_null().primary_key())
					.col(ColumnDef::new(TokenOnLedger::Decimals).tiny_integer().not_null())
					.col(ColumnDef::new(TokenOnLedger::EChainAmount).string().not_null())
					.col(ColumnDef::new(TokenOnLedger::SChainAmount).string().not_null())
					.col(ColumnDef::new(TokenOnLedger::HubAmount).string().not_null())
					.to_owned(),
			)
			.await
	}
}

#[derive(DeriveIden)]
enum TokenOnLedger {
	Table,
	ChainId,
	Decimals,
	TokenId,
	EChainAmount,
	SChainAmount,
	HubAmount,
}

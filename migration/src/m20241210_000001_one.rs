use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(TokenOnChain::Table)
					.col(ColumnDef::new(TokenOnChain::ChainId).string().not_null())
					.col(ColumnDef::new(TokenOnChain::TokenId).string().not_null())
					.col(ColumnDef::new(TokenOnChain::Amount).string().not_null())
					.primary_key(
						Index::create()
							.name("pk_chain_token_tvl")
							.col(TokenOnChain::ChainId)
							.col(TokenOnChain::TokenId)
							.primary(),
					)
					.to_owned(),
			)
			.await?;
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
			.await?;
		manager
			.create_table(
				Table::create()
					.table(TokenLedgerIdOnChain::Table)
					.col(ColumnDef::new(TokenLedgerIdOnChain::ChainId).string().not_null())
					.col(ColumnDef::new(TokenLedgerIdOnChain::TokenId).string().not_null())
					.col(ColumnDef::new(TokenLedgerIdOnChain::ContractId).string().not_null())
					.primary_key(
						Index::create()
							.name("pk_chain_token_contract_tvl")
							.col(TokenLedgerIdOnChain::ChainId)
							.col(TokenLedgerIdOnChain::TokenId)
							.primary(),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(TokenOnChain::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(TokenOnLedger::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(TokenLedgerIdOnChain::Table).to_owned())
			.await
	}
}

#[derive(DeriveIden)]
pub enum TokenOnChain {
	Table,
	ChainId,
	TokenId,
	Amount,
}

#[derive(DeriveIden)]
pub enum TokenLedgerIdOnChain {
	Table,
	ChainId,
	TokenId,
	ContractId,
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

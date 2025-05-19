use crate::entities::{token_on_chain, token_on_ledger};
use crate::entities::{token_on_chain::Entity as TokenOnChain, token_on_ledger::Entity as TokenOnLedger};
use log::info;
use sea_orm::DbConn;
use sea_orm::{sea_query::OnConflict, *};

pub struct Query;
impl Query {
	pub async fn get_all_amount_by_token(db: &DbConn, token_id: &str) -> Result<Vec<token_on_chain::Model>, DbErr> {
		TokenOnChain::find()
			.filter(Condition::all().add(token_on_chain::Column::TokenId.eq(token_id)))
			.all(db)
			.await
	}
	pub async fn get_token_amount_by_id(db: &DbConn, token_id: String, chain_id: String) -> Result<Option<token_on_chain::Model>, DbErr> {
		TokenOnChain::find_by_id((chain_id, token_id)).one(db).await
	}
}

pub struct Delete;
impl Delete {
	pub async fn remove_token_on_chains(db: &DbConn) -> Result<DeleteResult, DbErr> {
		TokenOnChain::delete_many()
			.filter(Condition::all().add(token_on_chain::Column::ChainId.is_not_null()))
			.exec(db)
			.await
	}

	pub async fn remove_token_on_ledgers(db: &DbConn) -> Result<DeleteResult, DbErr> {
		TokenOnLedger::delete_many()
			.filter(Condition::all().add(token_on_ledger::Column::ChainId.is_not_null()))
			.exec(db)
			.await
	}
}

pub struct Mutation;
impl Mutation {
	pub async fn save_token_on_chain(db: &DbConn, token_on_chain: token_on_chain::Model) -> Result<token_on_chain::Model, DbErr> {
		let active_model: token_on_chain::ActiveModel = token_on_chain.clone().into();
		let on_conflict = OnConflict::columns([token_on_chain::Column::ChainId, token_on_chain::Column::TokenId])
			.do_nothing()
			.to_owned();
		let insert_result = TokenOnChain::insert(active_model.clone()).on_conflict(on_conflict).exec(db).await;

		match insert_result {
			Ok(_ret) => {
				// info!("insert token on chain result : {:?}", ret);
			}
			Err(_) => {
				// let model = Self::update_token_on_chain(db, token_on_chain.clone(),
				// token_on_chain.clone().amount).await?; info!("the token on chain {:?}",
				// model);
			}
		}
		Ok(token_on_chain::Model { ..token_on_chain })
	}

	pub async fn save_token_on_ledger(db: &DbConn, token_on_ledger: token_on_ledger::Model) -> Result<token_on_ledger::Model, DbErr> {
		let active_model: token_on_ledger::ActiveModel = token_on_ledger.clone().into();
		let on_conflict = OnConflict::column(token_on_ledger::Column::TokenId).do_nothing().to_owned();
		let insert_result = TokenOnLedger::insert(active_model.clone()).on_conflict(on_conflict).exec(db).await;

		match insert_result {
			Ok(ret) => {
				info!("insert token on ledger result : {:?}", ret);
			}
			Err(_) => {
				let _model = Self::update_token_on_ledger(
					db,
					token_on_ledger.clone(),
					token_on_ledger.clone().e_chain_amount,
					token_on_ledger.clone().s_chain_amount,
					token_on_ledger.clone().hub_amount,
				)
				.await?;
				// info!("the token on ledger already exists, updated it ! {:?}", model);
			}
		}
		Ok(token_on_ledger::Model { ..token_on_ledger })
	}

	pub async fn update_token_on_chain(
		db: &DbConn,
		token_on_chain: token_on_chain::Model,
		amount: String,
	) -> Result<token_on_chain::Model, DbErr> {
		let mut active_model: token_on_chain::ActiveModel = token_on_chain.into();
		active_model.amount = Set(amount);
		let token_on_chain = active_model.update(db).await?;
		Ok(token_on_chain)
	}

	pub async fn update_token_on_ledger(
		db: &DbConn,
		token_on_ledger: token_on_ledger::Model,
		e_chain_amount: String,
		s_chain_amount: String,
		hub_amount: String,
	) -> Result<token_on_ledger::Model, DbErr> {
		let mut active_model: token_on_ledger::ActiveModel = token_on_ledger.into();
		active_model.e_chain_amount = Set(e_chain_amount);
		active_model.s_chain_amount = Set(s_chain_amount);
		active_model.hub_amount = Set(hub_amount);
		let token_on_ledger = active_model.update(db).await?;
		Ok(token_on_ledger)
	}
}

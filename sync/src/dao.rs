use crate::entities::token_on_ledger;
use crate::entities::token_on_ledger::Entity as TokenOnLedger;
use log::info;
use sea_orm::DbConn;
use sea_orm::{sea_query::OnConflict, *};

pub struct Mutation;
impl Mutation {
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
			}
		}
		Ok(token_on_ledger::Model { ..token_on_ledger })
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

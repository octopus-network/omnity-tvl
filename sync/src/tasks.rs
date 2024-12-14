use crate::entities::token_on_ledger;
use crate::{
	chains::*,
	types::{ChainId, OmnityTokenOnChain, TokenId},
	with_canister, Delete, Error as OmnityError, Mutation, Query,
};
use candid::{Decode, Encode};
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use log::error;
use log::info;
use sea_orm::DbConn;
use std::error::Error;
use std::{future::Future, sync::Arc, time::Duration};

pub const TOKEN_ON_CHAIN_SYNC_INTERVAL: u64 = 5;
pub const FETCH_LIMIT: u64 = 50;

pub fn spawn_sync_task<F, Fut>(db_conn: Arc<DbConn>, interval: u64, sync_fn: F) -> tokio::task::JoinHandle<()>
where
	F: Fn(Arc<DbConn>) -> Fut + Send + Sync + 'static,
	Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
	tokio::spawn(async move {
		let mut interval = tokio::time::interval(Duration::from_secs(interval));
		loop {
			sync_fn(db_conn.clone()).await.unwrap_or_else(|e| {
				error!("sync task error: {}", e);
			});
			interval.tick().await;
		}
	})
}

pub async fn execute_sync_tasks(db_conn: Arc<DbConn>) {
	let remove_database = async {
		let _ = Delete::remove_token_on_chains(&db_conn).await;
		let _ = Delete::remove_token_on_ledgers(&db_conn).await;
	};

	let sync_tokens_on_chains_from_hub =
		spawn_sync_task(db_conn.clone(), TOKEN_ON_CHAIN_SYNC_INTERVAL, |db_conn| async move {
			sync_tokens_on_chains(&db_conn).await
		});
	let sync_tokens_on_ledgers = spawn_sync_task(db_conn.clone(), TOKEN_ON_CHAIN_SYNC_INTERVAL, |db_conn| async move {
		sync_tokens_on_ledgers(&db_conn).await
	});
	let _ = tokio::join!(remove_database, sync_tokens_on_chains_from_hub, sync_tokens_on_ledgers);
}

pub async fn sync_tokens_on_chains(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing the hub ... ");

		let args = Encode!(&Vec::<u8>::new())?;
		let ret = agent
			.query(&canister_id, "get_token_position_size")
			.with_arg(args)
			.call()
			.await?;

		if let Ok(tokens_on_chains_size) = Decode!(&ret, Result<u64, OmnityError>)? {
			let mut from_seq = 0u64;

			while from_seq < tokens_on_chains_size {
				let tokens_on_chains_args = Encode!(&None::<ChainId>, &None::<TokenId>, &from_seq, &FETCH_LIMIT)?;
				let return_output = agent
					.query(&canister_id, "get_chain_tokens")
					.with_arg(tokens_on_chains_args)
					.call()
					.await?;

				if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
					if tokens_on_chains.is_empty() {
						break;
					}

					for _token_on_chain in tokens_on_chains.iter() {
						Mutation::save_token_on_chain(db, _token_on_chain.clone().into()).await?;
					}
					from_seq += tokens_on_chains.len() as u64;
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_tokens_on_ledgers(db: &DbConn) -> Result<(), Box<dyn Error>> {
	// 首先判断是哪种币，如果是icrc的，amount的就是ICP那边，
	// 然后以token_id为单位再找出token_ledger_id_on_chain同token_id有几条，
	// 每条用MATCH方式找出在各链的单位数。再相加一起
	with_canister("CKBTC_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKBTC canister ledgers... ");

		let ckbtc_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&ckbtc_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let ckbtc_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckBTC".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let osmosis = sync_with_osmosis(
			"factory%2Fosmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm%2FsICP-icrc-ckBTC",
		)
		.await?;
		let bitfinity = sync_with_bitfinity("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
		let e_amount = osmosis.parse::<u128>().unwrap() + bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKBTC".to_string(),
			8_i16,
			e_amount.to_string(),
			ckbtc_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await?;
	with_canister("EICP_HOPE_YOU_GET_RICH", |agent, canister_id| async move {
		info!("syncing tokens on HOPE_YOU_GET_RICH canister ledgers... ");

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent
			.query(&canister_id, "icrc1_total_supply")
			.with_arg(arg)
			.call()
			.await?;
		let _amount = Decode!(&ret, Nat)?.to_string().replace("_", "");
		// println!("{:?}", amount);
		Ok(())
	})
	.await
}

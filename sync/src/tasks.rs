use crate::{
	tokens::*,
	types::{ChainId, OmnityTokenOnChain, TokenId},
	with_canister, Error as OmnityError, Mutation,
};
use candid::{Decode, Encode};
use log::error;
use log::info;
use sea_orm::DbConn;
use std::error::Error;
use std::{future::Future, sync::Arc}; //time::Duration

pub const TOKEN_ON_CHAIN_SYNC_INTERVAL: u64 = 5;
pub const FETCH_LIMIT: u64 = 50;

pub fn spawn_sync_task<F, Fut>(db_conn: Arc<DbConn>, _interval: u64, sync_fn: F) -> tokio::task::JoinHandle<()>
where
	F: Fn(Arc<DbConn>) -> Fut + Send + Sync + 'static,
	Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
	tokio::spawn(async move {
		// let mut interval = tokio::time::interval(Duration::from_secs(interval));
		// loop {
		// 	sync_fn(db_conn.clone()).await.unwrap_or_else(|e| {
		// 		error!("sync task error: {}", e);
		// 	});
		// 	interval.tick().await;
		// }
		sync_fn(db_conn.clone()).await.unwrap_or_else(|e| {
			error!("sync task error: {}", e);
		});
	})
}

pub async fn execute_sync_tasks(db_conn: Arc<DbConn>) {
	// let remove_database = async {
	// 	let _ = Delete::remove_token_on_chains(&db_conn).await;
	// 	let _ = Delete::remove_token_on_ledgers(&db_conn).await;
	// };

	let sync_tokens_on_chains_from_hub =
		spawn_sync_task(db_conn.clone(), TOKEN_ON_CHAIN_SYNC_INTERVAL, |db_conn| async move {
			sync_tokens_on_chains(&db_conn).await
		});
	let sync_tokens_on_ledgers = spawn_sync_task(db_conn.clone(), TOKEN_ON_CHAIN_SYNC_INTERVAL, |db_conn| async move {
		sync_tokens_on_ledgers(&db_conn).await
	});
	let _ = tokio::join!(sync_tokens_on_chains_from_hub, sync_tokens_on_ledgers);
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
	// sync_dragginz(&db).await?;
	// sync_neuron_icp(&db).await?;
	// sync_cketh(&db).await?;
	// sync_ckusdt(&db).await?;
	sync_rune(&db, "ODINAPE_ID_BVAE_ODIN", "Bitcoin-runes-ODINAPE•ID•BVAE•ODIN", 8_i16).await?;
	sync_rune(&db, "ODINDOG_ID_YTTL_ODIN", "Bitcoin-runes-ODINDOG•ID•YTTL•ODIN", 8_i16).await?;
	sync_rune(&db, "ODINGOLD_ID_VACP_ODIN", "Bitcoin-runes-ODINGOLD•ID•VACP•ODIN", 8_i16).await?;
	sync_rune(&db, "SATOSHI_ID_OXTM_ODIN", "Bitcoin-runes-SATOSHI•ID•OXTM•ODIN", 8_i16).await?;
	sync_rune(&db, "ODINSTAS_ID_JXGT_ODIN", "Bitcoin-runes-ODINSTAS•ID•JXGT•ODIN", 8_i16).await?;
	sync_rune(&db, "BITCAT_ID_EOSE_ODIN", "Bitcoin-runes-BITCAT•ID•EOSE•ODIN", 8_i16).await?;
	sync_rune(&db, "ODINCAT_ID_DHGX_ODIN", "Bitcoin-runes-ODINCAT•ID•DHGX•ODIN", 8_i16).await?;
	sync_rune(&db, "FORSETISCN_ID_COIU_ODIN", "Bitcoin-runes-FORSETISCN•ID•COIU•ODIN", 8_i16).await?;
	sync_rune(&db, "PI_ID_YZHI_ODIN", "Bitcoin-runes-PI•ID•YZHI•ODIN", 8_i16).await?;
	sync_rune(&db, "RATS_ID_JXIT_ODIN", "Bitcoin-runes-RATS•ID•JXIT•ODIN", 8_i16).await?;
	sync_rune(&db, "ICONFUCIUS_ID_RVMN_ODIN", "Bitcoin-runes-ICONFUCIUS•ID•RVMN•ODIN", 8_i16).await?;
	sync_rune(&db, "DRAK_ID_HCNC_ODIN", "Bitcoin-runes-DRAK•ID•HCNC•ODIN", 8_i16).await?;
	sync_rune(&db, "SPARKS_ID_DTEH_ODIN", "Bitcoin-runes-SPARKS•ID•DTEH•ODIN", 8_i16).await?;
	sync_ckbtc(&db).await?;
	sync_icp(&db).await?;
	sync_rich(&db).await
}

use crate::tokens::*;
use log::error;
use sea_orm::DbConn;
use std::error::Error;
use std::{future::Future, sync::Arc};

pub const TOKEN_ON_CHAIN_SYNC_INTERVAL: u64 = 5;
pub const FETCH_LIMIT: u64 = 50;

pub fn spawn_sync_task<F, Fut>(db_conn: Arc<DbConn>, _interval: u64, sync_fn: F) -> tokio::task::JoinHandle<()>
where
	F: Fn(Arc<DbConn>) -> Fut + Send + Sync + 'static,
	Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
	tokio::spawn(async move {
		sync_fn(db_conn.clone()).await.unwrap_or_else(|e| {
			error!("sync task error: {}", e);
		});
	})
}

pub async fn execute_sync_tasks(db_conn: Arc<DbConn>) {
	let sync_tokens_on_ledgers = spawn_sync_task(db_conn.clone(), TOKEN_ON_CHAIN_SYNC_INTERVAL, |db_conn| async move {
		sync_tokens_on_ledgers(&db_conn).await
	});
	let _ = tokio::join!(sync_tokens_on_ledgers);
}

pub async fn sync_tokens_on_ledgers(db: &DbConn) -> Result<(), Box<dyn Error>> {
	// sync_dragginz(&db).await?;
	// sync_neuron_icp(&db).await?;
	// sync_cketh(&db).await?;
	// sync_ckusdt(&db).await?;

	// 13 + 2 + 4 + 5 + 4 = 28
	let runes = [
		("ODINAPE_ID_BVAE_ODIN", "Bitcoin-runes-ODINAPE•ID•BVAE•ODIN", 8_i16),
		("ODINDOG_ID_YTTL_ODIN", "Bitcoin-runes-ODINDOG•ID•YTTL•ODIN", 8_i16),
		("ODINGOLD_ID_VACP_ODIN", "Bitcoin-runes-ODINGOLD•ID•VACP•ODIN", 8_i16),
		("SATOSHI_ID_OXTM_ODIN", "Bitcoin-runes-SATOSHI•ID•OXTM•ODIN", 8_i16),
		("ODINSTAS_ID_JXGT_ODIN", "Bitcoin-runes-ODINSTAS•ID•JXGT•ODIN", 8_i16),
		("BITCAT_ID_EOSE_ODIN", "Bitcoin-runes-BITCAT•ID•EOSE•ODIN", 8_i16),
		("ODINCAT_ID_DHGX_ODIN", "Bitcoin-runes-ODINCAT•ID•DHGX•ODIN", 8_i16),
		("FORSETISCN_ID_COIU_ODIN", "Bitcoin-runes-FORSETISCN•ID•COIU•ODIN", 8_i16),
		("PI_ID_YZHI_ODIN", "Bitcoin-runes-PI•ID•YZHI•ODIN", 8_i16),
		("RATS_ID_JXIT_ODIN", "Bitcoin-runes-RATS•ID•JXIT•ODIN", 8_i16),
		("ICONFUCIUS_ID_RVMN_ODIN", "Bitcoin-runes-ICONFUCIUS•ID•RVMN•ODIN", 8_i16),
		("DRAK_ID_HCNC_ODIN", "Bitcoin-runes-DRAK•ID•HCNC•ODIN", 8_i16),
		("SPARKS_ID_DTEH_ODIN", "Bitcoin-runes-SPARKS•ID•DTEH•ODIN", 8_i16),
		("BITPANDA_ID_UUMF_ODIN", "Bitcoin-runes-BITPANDA•ID•UUMF•ODIN", 8_i16),
		("GHOSTNODE_ID_ZVVO_ODIN", "Bitcoin-runes-GHOSTNODE•ID•ZVVO•ODIN", 8_i16),
		("BITCAT_ID_YRMO_ODIN", "Bitcoin-runes-BITCAT•ID•YRMO•ODIN", 8_i16),
		("GOLDBTC_ID_PGZD_ODIN", "Bitcoin-runes-GOLDBTC•ID•PGZD•ODIN", 8_i16),
		("AIDEX_ID_AZNX_ODIN", "Bitcoin-runes-AIDEX•ID•AZNX•ODIN", 8_i16),
		("AOT_ID_GRMI_ODIN", "Bitcoin-runes-AOT•ID•GRMI•ODIN", 8_i16),
		("ODINPEPE_ID_HIRM_ODIN", "Bitcoin-runes-ODINPEPE•ID•HIRM•ODIN", 8_i16),
		("FLFWORL_ID_PUFE_ODIN", "Bitcoin-runes-FLFWORL•ID•PUFE•ODIN", 8_i16),
		("UDUCKLING_ID_WHRZ_ODIN", "Bitcoin-runes-UDUCKLING•ID•WHRZ•ODIN", 8_i16),
		("ODINBOT_ID_GIJQ_ODIN", "Bitcoin-runes-ODINBOT•ID•GIJQ•ODIN", 8_i16),
		("BITBULL_ID_VCZO_ODIN", "Bitcoin-runes-BITBULL•ID•VCZO•ODIN", 8_i16),
		("ICP_WORLD_COMPUTER", "Bitcoin-runes-ICP•WORLD•COMPUTER", 6_i16),
		("MAKE_CRYPTO_FUN_AGAIN", "Bitcoin-runes-MAKE•CRYPTO•FUN•AGAIN", 8_i16),
		("PROOF_OF_MEMES", "Bitcoin-runes-PROOF•OF•MEMES", 8_i16),
		("PUP_WIF_WOOF_OF_WORK", "Bitcoin-runes-PUP•WIF•WOOF•OF•WORK", 8_i16),
	];
	for (id, name, decimals) in runes {
		sync_rune(db, id, name, decimals).await?;
	}

	sync_ckbtc(db).await?;
	sync_icp(db).await?;
	sync_rich(db).await
}

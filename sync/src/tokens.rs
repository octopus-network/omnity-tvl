use crate::entities::token_on_ledger;
use crate::{
	chains::*,
	difference_warning,
	types::{ChainId, OmnityTokenOnChain},
	with_canister, Error as OmnityError, Mutation,
};
use anyhow::anyhow;
use candid::{Decode, Encode, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use log::{info, warn};
use sea_orm::DbConn;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

pub async fn sync_ckbtc(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("CKBTC_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKBTC canister ledgers... ");

		let ckbtc_token_id = "sICP-icrc-ckBTC";
		let ckbtc_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&ckbtc_reqst)?;
		let ret = agent.query(&canister_id, "icrc1_balance_of").with_arg(arg).call().await?;
		let ckbtc_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		let mut count = 0;
		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let count1 = Arc::new(Mutex::new(0usize));
		while hub_amount == 0 {
			while count != 5 {
				let amount_clone = hub_amount1.clone();
				let count_clone = count1.clone();
				let _ = with_canister("OMNITY_HUB_CANISTER_ID", |hub_agent, hub_canister_id| async move {
					let tokens_on_chains_args = Encode!(&None::<ChainId>, &ckbtc_token_id.to_string(), &0u64, &100_u64)?;
					let return_output = hub_agent
						.query(&hub_canister_id, "get_chain_tokens")
						.with_arg(tokens_on_chains_args)
						.call()
						.await?;

					if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
						if !tokens_on_chains.is_empty() {
							*count_clone.lock().await = tokens_on_chains.len();
							for tamount in tokens_on_chains {
								*amount_clone.lock().await += tamount.amount
							}
						}
					}
					Ok(())
				})
				.await?;
				count = *count1.lock().await;
			}
			break;
		}
		hub_amount = *hub_amount1.lock().await;

		let osmosis =
			sync_with_osmosis("factory%2Fosmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm%2FsICP-icrc-ckBTC").await?;
		let bitfinity = sync_with_bitfinity("0x56bf74ef5d4ad161d2d8d5d576e70108f152cd35").await?;
		let ton = sync_with_ton("EQD3IJCxBHFRNCFFLmtnoIyMEYt_Zio3WT0YQQujA2tSuCTZ").await?;
		let core = sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://rpc.ankr.com/core").await?;

		let osmosis_supply = osmosis.parse::<u128>().unwrap_or_default();
		let bitfinity_supply = bitfinity.parse::<u128>().unwrap_or_default();
		let ton_supply = ton.parse::<u128>().unwrap_or_default();
		let core_supply = core.parse::<u128>().unwrap_or_default();

		info!("ton ckbtc : {:?}", ton_supply);
		info!("bitfinity ckbtc : {:?}", bitfinity_supply);
		info!("core ckbtc : {:?}", core_supply);
		info!("osmosis ckbtc : {:?}", osmosis_supply);

		let e_amount = osmosis_supply + bitfinity_supply + ton_supply + core_supply;
		info!("ckBTC e_chain_amount: {:?}", &e_amount);
		info!("ckBTC s_chain_amount: {:?}", &ckbtc_amount);
		info!("ckBTC hub_amount: {:?}", &hub_amount);

		let ckbtc_amount_u128 = ckbtc_amount.parse::<u128>().unwrap_or(0);
		info!(
			"ckBTC S-E 差异: {:?}, 目前比例 {:?} %",
			&ckbtc_amount_u128 - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(ckbtc_amount_u128))
				.unwrap_or_default()
		);
		info!(
			"ckBTC S-H 差异: {:?}, 目前比例 {:?} %",
			&ckbtc_amount_u128 - &hub_amount,
			&hub_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(ckbtc_amount_u128))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKBTC".to_string(),
			8_i16,
			e_amount.to_string(),
			ckbtc_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && ckbtc_amount_u128 != 0 && hub_amount != 0 {
			if difference_warning(e_amount, ckbtc_amount_u128, hub_amount) {
				warn!("CKBTC 差距大了 ！！！");
				// e>s 确认不好, e<s 确认可以，H大S小/S小H大/H大E小/S小E大 分别对应场景?
				if e_amount > ckbtc_amount_u128 {
					if (e_amount - ckbtc_amount_u128) as f64 / e_amount as f64 > 0.01 {
						warn!("ckbtc difference is greater than 1%");
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn pause_hub() -> Result<(), Box<dyn Error>> {
	with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
		warn!("Trying to pause Omnity Hub ... ");
		let arg: Vec<u8> = Encode!(&Vec::<u8>::new())?;
		let ret = agent.query(&canister_id, "paused").with_arg(arg.clone()).call().await?;
		let is_paused = Decode!(&ret, bool)?;

		if !is_paused {
			match agent.update(&canister_id, "audit_stop").with_arg(arg).call_and_wait().await {
				Ok(_ret) => {
					info!("Complete to pause Omnity Hub ...");
				}
				Err(e) => {
					info!("err to pause Omnity Hub ... {:?}", e);
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_icp(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("ICP_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on ICP canister ledgers... ");
		let icp_token_id = "sICP-native-ICP";

		let icp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&icp_reqst)?;
		let ret = agent.query(&canister_id, "icrc1_balance_of").with_arg(arg).call().await?;
		let icp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let amount_clone = hub_amount1.clone();
		let _ = with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
			let tokens_on_chains_args = Encode!(&None::<ChainId>, &icp_token_id.to_string(), &0u64, &100_u64)?;
			let return_output = agent
				.query(&canister_id, "get_chain_tokens")
				.with_arg(tokens_on_chains_args)
				.call()
				.await?;

			if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
				if !tokens_on_chains.is_empty() {
					for tamount in tokens_on_chains {
						*amount_clone.lock().await += tamount.amount
					}
				}
			}
			Ok(())
		})
		.await?;
		let hub_amount = *hub_amount1.lock().await;

		let osmosis = sync_with_osmosis("factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-native-ICP").await?;
		let bitfinity = sync_with_bitfinity("0x51cCdE9Ca75d95BB55eCe1775fCBFF91324B18A6").await?;
		let ethereum = sync_with_ethereum("0x8e6e7cd8db9c9b73c6c6221702146840b12d6763", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
		let ton = sync_with_ton("EQCW0ddLCQAn011bb8T2Xdoa40v6A_bL3cfjn0bplXdSKnWa").await?;
		let sui = sync_with_sui("0x1c437c7a6acc30d1e1249dbc0bc53dc6f5e1803261bd176d88dec25bc8548af3::icp::ICP").await?;
		let base = sync_with_eth_call("0x56bf74ef5d4ad161d2d8d5d576e70108f152cd35", "https://base-pokt.nodies.app").await?;
		let solana = sync_with_solana("79yjxQmS7NWd3a5ZDrVrVcP9xEPsT4tFCys5SUdG8VxN").await?;
		////目前OSMOSIS占大头，不会低于1%，一旦这个占比小了，其它3条小链Ton/eSui/eSolana会小于1%以及暂停
		let osmosis_supply = osmosis.parse::<u128>().unwrap_or_default();
		let bitfinity_supply = bitfinity.parse::<u128>().unwrap_or_default();
		let ethereum_supply = ethereum.parse::<u128>().unwrap_or_default();
		let ton_supply = ton.parse::<u128>().unwrap_or_default();
		let sui_supply = sui.parse::<u128>().unwrap_or_default();
		let base_supply = base.parse::<u128>().unwrap_or_default();
		let solana_supply = solana.parse::<u128>().unwrap_or_default();

		let e_amount = osmosis_supply + bitfinity_supply + ethereum_supply + ton_supply + sui_supply + base_supply + solana_supply;

		info!("ton icp : {:?}", ton_supply);
		info!("bitfinity icp : {:?}", bitfinity_supply);
		info!("ethereum icp : {:?}", ethereum_supply);
		info!("osmosis icp : {:?}", osmosis_supply);
		info!("sui icp : {:?}", sui_supply);
		info!("base icp : {:?}", base_supply);
		info!("solana icp : {:?}", solana_supply);

		info!("ICP e_chain_amount: {:?}", &e_amount);
		info!("ICP s_chain_amount: {:?}", &icp_amount);
		info!("ICP hub_amount: {:?}", &hub_amount);

		let icp_amount_u128 = icp_amount.parse::<u128>().unwrap_or(0);
		info!(
			"ICP S-E 差异: {:?}, 目前比例 {:?} %",
			&icp_amount_u128 - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(icp_amount_u128))
				.unwrap_or_default()
		);
		info!(
			"ICP H-S 差异: {:?}, 目前比例 {:?} %",
			&hub_amount - &icp_amount_u128,
			&icp_amount_u128
				.checked_mul(100)
				.and_then(|n| n.checked_div(hub_amount))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"ICP".to_string(),
			8_i16,
			e_amount.to_string(),
			icp_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		if e_amount != 0 && icp_amount_u128 != 0 && hub_amount != 0 {
			if difference_warning(e_amount, icp_amount_u128, hub_amount) {
				warn!("ICP 差距大了 ！！！");
				// e>s 确认不好, e<s 确认可以，H大S小/S小H大/H大S小/S小H大 分别对应场景?
				if e_amount > icp_amount_u128 {
					if (e_amount - icp_amount_u128) as f64 / e_amount as f64 > 0.01 {
						warn!("icp difference is greater than 1%");
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_rich(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("EICP_HOPE_YOU_GET_RICH", |agent, canister_id| async move {
		info!("syncing tokens on HOPE_YOU_GET_RICH canister ledgers... ");
		let rich_token_id = "Bitcoin-runes-HOPE•YOU•GET•RICH";

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent.query(&canister_id, "icrc1_total_supply").with_arg(arg).call().await?;
		let eicp = Decode!(&ret, Nat)?.to_string().replace("_", "");
		let bitfinity = sync_with_bitfinity("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
		let ailayer = sync_with_eth_call("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4", "https://mainnet-rpc.ailayer.xyz").await?;
		let bitlayer = sync_with_bitlayer("0xb32b737817ba8ff81c696ca8fbd4832cca5751a6").await?;
		let bsquared = sync_with_bsquared("0x20dD93ad6675E81a635C7be034dC1C9Ce0AE2DE4").await?;
		let bevm = sync_with_bevm("0xB76fD1B6CDA18a8cFA255E23059c0bB1624bB5F9").await?;
		let bob = sync_with_bob("0x8f9568BB47b7772f334CcceF4652C9ac7678f21a").await?;
		let ethereum = sync_with_ethereum("0xD14fAd0Fe8175aFD3f4c22B25736E11CF42341A5", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
		let ton = sync_with_ton("EQBGKSkJ307rZY46kqSwwmHskOwSPEO5urm5EZ_EWFyk3bEO").await?;
		let solana = sync_with_solana("8j45TBhQU6DQhRvoYd9dpQWzTNKstB6kpnfZ3pKDCxff").await?;

		let key = std::env::var("ALCHEMY_KEY")
			.map_err(|_| anyhow!("LCHEMY_KEY is not found"))
			.unwrap();
		let url = "https://rootstock-mainnet.g.alchemy.com/v2/".to_string() + &key;
		let rootstock = sync_with_eth_call("0xb943b047473218a8e0fc637e96136071ffa3f842", &url).await?;

		let xlayer = sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://xlayer.drpc.org").await?;
		let merlin = sync_with_eth_call("0xfd4de66eca49799bdde66eb33654e2198ab7bba4", "https://rpc.merlinchain.io").await?;
		let core = sync_with_eth_call("0xfd4de66eca49799bdde66eb33654e2198ab7bba4", "https://rpc.ankr.com/core").await?;
		let base = sync_with_eth_call("0xfd4de66eca49799bdde66eb33654e2198ab7bba4", "https://base-pokt.nodies.app").await?;
		//目前eICP占大头，不会低于1%，一旦这个占比小了，Ton/eSolana/会小于1%, 15 chains
		let eicp_supply = eicp.parse::<u128>().unwrap_or_default();
		let bitfinity_supply = bitfinity.parse::<u128>().unwrap_or_default();
		let ailayer_supply = ailayer.parse::<u128>().unwrap_or_default();
		let bitlayer_supply = bitlayer.parse::<u128>().unwrap_or_default();
		let bsquared_supply = bsquared.parse::<u128>().unwrap_or_default();
		let bevm_supply = bevm.parse::<u128>().unwrap_or_default();
		let bob_supply = bob.parse::<u128>().unwrap_or_default();
		let ethereum_supply = ethereum.parse::<u128>().unwrap_or_default();
		let ton_supply = ton.parse::<u128>().unwrap_or_default();
		let solana_supply = solana.parse::<u128>().unwrap_or_default();
		let rootstock_supply = rootstock.parse::<u128>().unwrap_or_default();
		let xlayer_supply = xlayer.parse::<u128>().unwrap_or_default();
		let merlin_supply = merlin.parse::<u128>().unwrap_or_default();
		let core_supply = core.parse::<u128>().unwrap_or_default();
		let base_supply = base.parse::<u128>().unwrap_or_default();
		info!("solana Rich : {:?}", solana_supply);
		info!("bob Rich : {:?}", bob_supply);
		info!("rootstock Rich : {:?}", rootstock_supply);
		info!("ethereum Rich : {:?}", ethereum_supply);
		info!("bevm Rich : {:?}", bevm_supply);
		info!("xlayer Rich : {:?}", xlayer_supply);
		info!("merlin Rich : {:?}", merlin_supply);
		info!("ailayer Rich : {:?}", ailayer_supply);
		info!("eicp Rich : {:?}", eicp_supply);
		info!("bitfinity Rich : {:?}", bitfinity_supply);
		info!("bsquared Rich : {:?}", bsquared_supply);
		info!("ton Rich : {:?}", ton_supply);
		info!("bitlayer Rich : {:?}", bitlayer_supply);
		info!("core Rich : {:?}", core_supply);
		info!("base Rich : {:?}", base_supply);

		let e_amount =
			eicp_supply
				+ bitfinity_supply
				+ ailayer_supply
				+ bitlayer_supply
				+ bsquared_supply
				+ bevm_supply
				+ bob_supply + ethereum_supply
				+ ton_supply + solana_supply
				+ rootstock_supply
				+ xlayer_supply
				+ merlin_supply
				+ core_supply
				+ base_supply;

		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let amount_clone = hub_amount1.clone();
		let _ = with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
			let tokens_on_chains_args = Encode!(&None::<ChainId>, &rich_token_id.to_string(), &0u64, &100_u64)?;
			let return_output = agent
				.query(&canister_id, "get_chain_tokens")
				.with_arg(tokens_on_chains_args)
				.call()
				.await?;

			if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
				if !tokens_on_chains.is_empty() {
					for tamount in tokens_on_chains {
						*amount_clone.lock().await += tamount.amount
					}
				}
			}
			Ok(())
		})
		.await?;
		let hub_amount = *hub_amount1.lock().await;

		let s_chain_amount1 = Arc::new(Mutex::new(0u128));
		let s_chain_amount_clone = s_chain_amount1.clone();
		let _ = with_canister("OMNITY_CUSTOMS_BITCOIN_CANISTER_ID", |agent, canister_id| async move {
			let rune_token_lock_args = Encode!(&rich_token_id.to_string())?;
			let token_lock_return_output = agent
				.query(&canister_id, "token_lock_amount")
				.with_arg(rune_token_lock_args)
				.call()
				.await?;

			let rune_amount = Decode!(&token_lock_return_output, u128)?;
			*s_chain_amount_clone.lock().await = rune_amount;

			Ok(())
		})
		.await?;
		let s_chain_amount = *s_chain_amount1.lock().await;

		info!("RICH e_chain_amount: {:?}", &e_amount);
		info!("RICH s_chain_amount: {:?}", &s_chain_amount);
		info!("RICH hub_amount: {:?}", &hub_amount);
		info!(
			"RICH S-E 差异: {:?}, 目前比例 {:?} %",
			&s_chain_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(s_chain_amount))
				.unwrap_or_default()
		);

		info!(
			"RICH H-E 差异: {:?} 目前比例 {:?} %",
			&hub_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(hub_amount))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"RUNES".to_string(),
			"HOPE•YOU•GET•RICH".to_string(),
			2_i16,
			e_amount.to_string(),
			s_chain_amount.to_string(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && s_chain_amount != 0 && hub_amount != 0 {
			if difference_warning(e_amount, s_chain_amount, hub_amount) {
				warn!("RICH 差距大了！！！");
				// e>s 确认不好, e<s 确认可以，H大S小/S小H大/H大E小/S小E大 分别对应场景?
				// 目前E>S，但只有0.22%
				if e_amount > s_chain_amount {
					if (e_amount - s_chain_amount) as f64 / e_amount as f64 > 0.01 {
						warn!("RICH difference is greater than 1%");
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_rune(db: &DbConn, canister: &str, token: &str, decimal: i16) -> Result<(), Box<dyn Error>> {
	with_canister(canister, |agent, canister_id| async move {
		info!("syncing tokens on {:?} canister ledgers... ", canister);

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent.query(&canister_id, "icrc1_total_supply").with_arg(arg).call().await?;
		let eicp = Decode!(&ret, Nat)?.to_string().replace("_", "");
		let eicp_supply = eicp.parse::<u128>().unwrap_or_default();

		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let amount_clone = hub_amount1.clone();
		let _ = with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
			let tokens_on_chains_args = Encode!(&"eICP".to_string(), &token.to_string(), &0u64, &10_u64)?;
			let return_output = agent
				.query(&canister_id, "get_chain_tokens")
				.with_arg(tokens_on_chains_args)
				.call()
				.await?;

			if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
				if !tokens_on_chains.is_empty() {
					*amount_clone.lock().await = tokens_on_chains[0].amount;
				}
			}
			Ok(())
		})
		.await?;
		let hub_amount = *hub_amount1.lock().await;

		let s_chain_amount1 = Arc::new(Mutex::new(0u128));
		let s_chain_amount_clone = s_chain_amount1.clone();
		let _ = with_canister("OMNITY_CUSTOMS_BITCOIN_CANISTER_ID", |agent, canister_id| async move {
			let rune_token_lock_args = Encode!(&token.to_string())?;
			let token_lock_return_output = agent
				.query(&canister_id, "token_lock_amount")
				.with_arg(rune_token_lock_args)
				.call()
				.await?;

			let rune_amount = Decode!(&token_lock_return_output, u128)?;
			*s_chain_amount_clone.lock().await = rune_amount;

			Ok(())
		})
		.await?;
		let s_chain_amount = *s_chain_amount1.lock().await;

		info!("{:?} e_chain_amount: {:?}", &canister, &eicp_supply);
		info!("{:?} s_chain_amount: {:?}", &canister, &s_chain_amount);
		info!("{:?} hub_amount: {:?}", &canister, &hub_amount);
		info!(
			"{:?} S-E 差异: {:?}, 目前比例 {:?} %",
			&canister,
			&s_chain_amount - &eicp_supply,
			&eicp_supply
				.checked_mul(100)
				.and_then(|n| n.checked_div(s_chain_amount))
				.unwrap_or_default()
		);
		info!(
			"{:?} H-E 差异: {:?} 目前比例 {:?} %",
			&canister,
			&hub_amount.saturating_sub(eicp_supply),
			&eicp_supply
				.checked_mul(100)
				.and_then(|n| n.checked_div(hub_amount))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"RUNES".to_string(),
			token.to_string(),
			decimal,
			eicp_supply.to_string(),
			s_chain_amount.to_string(),
			hub_amount.clone().to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if eicp_supply != 0 && s_chain_amount != 0 && hub_amount != 0 {
			if difference_warning(eicp_supply, s_chain_amount, hub_amount) {
				warn!("{:?} 差距大了 ！！！", &canister);
				// e>s 确认不好, e<s 确认可以，H大S小/S小H大/H大E小/S小E大 分别对应场景?
				// ODINDOG_ID_YTTL_ODIN小0.01%/BITCAT_ID_EOSE_ODIN小0.001%/GHOSTNODE_ID_ZVVO_ODIN
				if eicp_supply > s_chain_amount {
					if (eicp_supply - s_chain_amount) as f64 / eicp_supply as f64 > 0.01 {
						warn!("{:?} difference is greater than 1%", &canister);
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_runes_x_bitcoin(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("EICP_RUNES_X_BITCOIN", |agent, canister_id| async move {
		info!("syncing tokens on RUNES•X•BITCOIN.OT canister ledgers... ");
		let runes_x_bitcoin_token_id = "Bitcoin-runes-RUNES•X•BITCOIN";

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent.query(&canister_id, "icrc1_total_supply").with_arg(arg).call().await?;
		let eicp = Decode!(&ret, Nat)?.to_string().replace("_", "");
		let bitlayer = sync_with_bitlayer("0xfdd3173d2c3defa7a3ac6c08ad0f03dc9eceb230").await?;
		let ethereum = sync_with_ethereum("0xbe175b13f733e8b57382717597dbd8205e6fd24a", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
		// let bob = sync_with_bob("0x6012fcafe3f570dcd491da3546ff748eb9308146").await?;

		let eicp_supply = eicp.parse::<u128>().unwrap_or_default();
		let bitlayer_supply = bitlayer.parse::<u128>().unwrap_or_default();
		let ethereum_supply = ethereum.parse::<u128>().unwrap_or_default();
		// let bob_supply = bob.parse::<u128>().unwrap_or_default();

		info!("eicp RUNES•X•BITCOIN : {:?}", eicp_supply);
		info!("bitlayer RUNES•X•BITCOIN : {:?}", bitlayer_supply);
		info!("ethereum RUNES•X•BITCOIN : {:?}", ethereum_supply);
		// info!("bob RUNES•X•BITCOIN : {:?}", bob_supply);

		let e_amount = eicp_supply + bitlayer_supply + ethereum_supply; // + bob_supply;
		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let amount_clone = hub_amount1.clone();
		let _ = with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
			let tokens_on_chains_args = Encode!(&None::<ChainId>, &runes_x_bitcoin_token_id.to_string(), &0u64, &100_u64)?;
			let return_output = agent
				.query(&canister_id, "get_chain_tokens")
				.with_arg(tokens_on_chains_args)
				.call()
				.await?;

			if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
				if !tokens_on_chains.is_empty() {
					for tamount in tokens_on_chains {
						*amount_clone.lock().await += tamount.amount
					}
				}
			}
			Ok(())
		})
		.await?;
		let hub_amount = *hub_amount1.lock().await;

		let s_chain_amount1 = Arc::new(Mutex::new(0u128));
		let s_chain_amount_clone = s_chain_amount1.clone();
		let _ = with_canister("OMNITY_CUSTOMS_BITCOIN_CANISTER_ID", |agent, canister_id| async move {
			let rune_token_lock_args = Encode!(&runes_x_bitcoin_token_id.to_string())?;
			let token_lock_return_output = agent
				.query(&canister_id, "token_lock_amount")
				.with_arg(rune_token_lock_args)
				.call()
				.await?;

			let rune_amount = Decode!(&token_lock_return_output, u128)?;
			*s_chain_amount_clone.lock().await = rune_amount;

			Ok(())
		})
		.await?;
		let s_chain_amount = *s_chain_amount1.lock().await;

		info!("RUNES•X•BITCOIN e_chain_amount: {:?}", &e_amount);
		info!("RUNES•X•BITCOIN s_chain_amount: {:?}", &s_chain_amount);
		info!("RUNES•X•BITCOIN hub_amount: {:?}", &hub_amount);
		info!(
			"RUNES•X•BITCOIN S-E 差异: {:?}, 目前比例 {:?} %",
			&s_chain_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(s_chain_amount))
				.unwrap_or_default()
		);

		info!(
			"RUNES•X•BITCOIN H-E 差异: {:?} 目前比例 {:?} %",
			&hub_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(hub_amount))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"RUNES".to_string(),
			"RUNES•X•BITCOIN".to_string(),
			0_i16,
			e_amount.to_string(),
			s_chain_amount.to_string(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && s_chain_amount != 0 && hub_amount != 0 {
			if difference_warning(e_amount, s_chain_amount, hub_amount) {
				warn!("RUNES•X•BITCOIN 差距大了！！！");
				if e_amount > s_chain_amount {
					if (e_amount - s_chain_amount) as f64 / e_amount as f64 > 0.01 {
						warn!("RUNES•X•BITCOIN difference is greater than 1%");
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

pub async fn sync_dog_go_to_the_moon(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("EICP_DOG_GO_TO_THE_MOON", |agent, canister_id| async move {
		info!("syncing tokens on DOG•GO•TO•THE•MOON canister ledgers... ");
		let dog_go_to_the_moon_token_id = "Bitcoin-runes-DOG•GO•TO•THE•MOON";

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent.query(&canister_id, "icrc1_total_supply").with_arg(arg).call().await?;
		let eicp = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let base = sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://base-pokt.nodies.app").await?;
		let bevm = sync_with_bevm("0xbe175b13f733e8b57382717597dbd8205e6fd24a").await?;
		let xlayer = sync_with_eth_call("0x56bf74ef5d4ad161d2d8d5d576e70108f152cd35", "https://xlayer.drpc.org").await?;
		// let bitfinity = sync_with_bitfinity("0x44b74e57a9ef3828f1eb8d47c42acdf6ce1445b8").await?;
		let key = std::env::var("ALCHEMY_KEY")
			.map_err(|_| anyhow!("LCHEMY_KEY is not found"))
			.unwrap();
		let url = "https://rootstock-mainnet.g.alchemy.com/v2/".to_string() + &key;
		let rootstock = sync_with_eth_call("0xfd4de66eca49799bdde66eb33654e2198ab7bba4", &url).await?;
		let osmosis =
			sync_with_osmosis("factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/Bitcoin-runes-DOG.GO.TO.THE.MOON")
				.await?;
		let bsquared = sync_with_bsquared("0x5ea478b64c43c683d692a3016ce07550565e929a").await?;
		let ton = sync_with_ton("EQBlHAxiJMi8EpfeuKX4NAzvBcJdN20MBIWjUZBmIq21NdbJ").await?;
		let ethereum = sync_with_ethereum("0x2fd9afe2589b6bb44c61ca8e0620a43070efb941", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
		let ailayer = sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://mainnet-rpc.ailayer.xyz").await?;
		let merlin = sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://rpc.merlinchain.io").await?;
		let bitlayer = sync_with_bitlayer("0x90a75e214bda302196cb8279c1331320579e3d91").await?;
		// let bob = sync_with_bob("0xeb95424bd91dbd735db0bcd6ece191ef2e24d286").await?;
		let core = sync_with_eth_call("0x3662afef38c94a6184cdfce8dcc60e7c305b8299", "https://rpc.ankr.com/core").await?;

		let eicp_supply = eicp.parse::<u128>().unwrap_or_default();
		// let bitfinity_supply = bitfinity.parse::<u128>().unwrap_or_default();
		let ailayer_supply = ailayer.parse::<u128>().unwrap_or_default();
		let bsquared_supply = bsquared.parse::<u128>().unwrap_or_default();
		let bevm_supply = bevm.parse::<u128>().unwrap_or_default();
		// let bob_supply = bob.parse::<u128>().unwrap_or_default();
		let ethereum_supply = ethereum.parse::<u128>().unwrap_or_default();
		let ton_supply = ton.parse::<u128>().unwrap_or_default();
		let rootstock_supply = rootstock.parse::<u128>().unwrap_or_default();
		let xlayer_supply = xlayer.parse::<u128>().unwrap_or_default();
		let merlin_supply = merlin.parse::<u128>().unwrap_or_default();
		let bitlayer_supply = bitlayer.parse::<u128>().unwrap_or_default();
		let core_supply = core.parse::<u128>().unwrap_or_default();
		let base_supply = base.parse::<u128>().unwrap_or_default();
		let osmosis_supply = osmosis.parse::<u128>().unwrap_or_default();
		// info!("bob DOG•GO•TO•THE•MOON : {:?}", bob_supply);
		info!("rootstock DOG•GO•TO•THE•MOON : {:?}", rootstock_supply);
		info!("ethereum DOG•GO•TO•THE•MOON : {:?}", ethereum_supply);
		info!("bevm DOG•GO•TO•THE•MOON : {:?}", bevm_supply);
		info!("xlayer DOG•GO•TO•THE•MOON : {:?}", xlayer_supply);
		info!("merlin DOG•GO•TO•THE•MOON : {:?}", merlin_supply);
		info!("ailayer DOG•GO•TO•THE•MOON : {:?}", ailayer_supply);
		info!("eicp DOG•GO•TO•THE•MOON : {:?}", eicp_supply);
		// info!("bitfinity DOG•GO•TO•THE•MOON : {:?}", bitfinity_supply);
		info!("bsquared DOG•GO•TO•THE•MOON : {:?}", bsquared_supply);
		info!("ton DOG•GO•TO•THE•MOON : {:?}", ton_supply);
		info!("bitlayer DOG•GO•TO•THE•MOON : {:?}", bitlayer_supply);
		info!("core DOG•GO•TO•THE•MOON : {:?}", core_supply);
		info!("base DOG•GO•TO•THE•MOON : {:?}", base_supply);
		info!("osmosis DOG•GO•TO•THE•MOON : {:?}", osmosis_supply);

		let e_amount = eicp_supply
				// + bitfinity_supply
				+ ailayer_supply
				+ bitlayer_supply
				+ bsquared_supply
				+ bevm_supply
				// + bob_supply 
				+ ethereum_supply
				+ ton_supply + rootstock_supply
				+ xlayer_supply
				+ merlin_supply
				+ core_supply
				+ base_supply
				+ osmosis_supply;

		let hub_amount1 = Arc::new(Mutex::new(0u128));
		let amount_clone = hub_amount1.clone();
		let _ = with_canister("OMNITY_HUB_CANISTER_ID", |agent, canister_id| async move {
			let tokens_on_chains_args = Encode!(&None::<ChainId>, &dog_go_to_the_moon_token_id.to_string(), &0u64, &100_u64)?;
			let return_output = agent
				.query(&canister_id, "get_chain_tokens")
				.with_arg(tokens_on_chains_args)
				.call()
				.await?;

			if let Ok(tokens_on_chains) = Decode!(&return_output, Result<Vec<OmnityTokenOnChain>, OmnityError>)? {
				if !tokens_on_chains.is_empty() {
					for tamount in tokens_on_chains {
						*amount_clone.lock().await += tamount.amount
					}
				}
			}
			Ok(())
		})
		.await?;
		let hub_amount = *hub_amount1.lock().await;

		let s_chain_amount1 = Arc::new(Mutex::new(0u128));
		let s_chain_amount_clone = s_chain_amount1.clone();
		let _ = with_canister("OMNITY_CUSTOMS_BITCOIN_CANISTER_ID", |agent, canister_id| async move {
			let rune_token_lock_args = Encode!(&dog_go_to_the_moon_token_id.to_string())?;
			let token_lock_return_output = agent
				.query(&canister_id, "token_lock_amount")
				.with_arg(rune_token_lock_args)
				.call()
				.await?;

			let rune_amount = Decode!(&token_lock_return_output, u128)?;
			*s_chain_amount_clone.lock().await = rune_amount;

			Ok(())
		})
		.await?;
		let s_chain_amount = *s_chain_amount1.lock().await;

		info!("DOG•GO•TO•THE•MOON e_chain_amount: {:?}", &e_amount);
		info!("DOG•GO•TO•THE•MOON s_chain_amount: {:?}", &s_chain_amount);
		info!("DOG•GO•TO•THE•MOON hub_amount: {:?}", &hub_amount);
		info!(
			"DOG•GO•TO•THE•MOON S-E 差异: {:?}, 目前比例 {:?} %",
			&s_chain_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(s_chain_amount))
				.unwrap_or_default()
		);

		info!(
			"DOG•GO•TO•THE•MOON H-E 差异: {:?} 目前比例 {:?} %",
			&hub_amount - &e_amount,
			&e_amount
				.checked_mul(100)
				.and_then(|n| n.checked_div(hub_amount))
				.unwrap_or_default()
		);

		let token_on_ledger = token_on_ledger::Model::new(
			"RUNES".to_string(),
			"DOG•GO•TO•THE•MOON".to_string(),
			5_i16,
			e_amount.to_string(),
			s_chain_amount.to_string(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && s_chain_amount != 0 && hub_amount != 0 {
			if difference_warning(e_amount, s_chain_amount, hub_amount) {
				warn!("DOG•GO•TO•THE•MOON 差距大了！！！");
				if e_amount > s_chain_amount {
					if (e_amount - s_chain_amount) as f64 / e_amount as f64 > 0.01 {
						warn!("DOG•GO•TO•THE•MOON difference is greater than 1%");
						let _ = pause_hub().await?;
					}
				}
			}
		}
		Ok(())
	})
	.await
}

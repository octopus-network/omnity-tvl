use crate::entities::token_on_ledger;
use crate::{chains::*, difference_warning, with_canister, Mutation, Query};
use candid::{Decode, Encode};
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use log::{info, warn};
use sea_orm::DbConn;
use std::error::Error;

pub async fn sync_cketh(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("CKETH_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKETH canister ledgers... ");

		let cketh_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&cketh_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let cketh_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckETH".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x242BbcB4f4F1b752Ae30757DC9AE9C24d9A9B640").await?;
		info!("bitfinity ckETH : {:?}", bitfinity);

		let e_amount = bitfinity.parse::<u128>().unwrap_or_default();

		info!("ckETH e_chain_amount: {:?}", &e_amount);
		info!("ckETH s_chain_amountt: {:?}", &cketh_amount);
		info!("ckETH hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKETH".to_string(),
			18_i16,
			e_amount.to_string(),
			cketh_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && cketh_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, cketh_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("ckETH difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_ckbtc(db: &DbConn) -> Result<(), Box<dyn Error>> {
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
		let mut count = 0;

		while hub_amount == 0 {
			while count != 5 {
				if let Ok(ckbtc_amounts) = Query::get_all_amount_by_token(db, "sICP-icrc-ckBTC".to_string()).await {
					count = ckbtc_amounts.len();
					if ckbtc_amounts.len() == 5 {
						for tamount in &ckbtc_amounts {
							if let Ok(amt) = tamount.amount.parse::<u128>() {
								hub_amount += amt;
							}
						}
					}
				}
			}
			break;
		}

		let osmosis = sync_with_osmosis(
			"factory%2Fosmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm%2FsICP-icrc-ckBTC",
		)
		.await?;
		let bitfinity = sync_with_bitfinity("0x56bf74ef5d4ad161d2d8d5d576e70108f152cd35").await?;
		let ton = sync_with_ton("EQD3IJCxBHFRNCFFLmtnoIyMEYt_Zio3WT0YQQujA2tSuCTZ").await?;
		let core = sync_with_core(
			"0x51ccde9ca75d95bb55ece1775fcbff91324b18a6",
			"9ede2feeb2404baabaa4254590950ec6",
		)
		.await?;
		info!("ton ckbtc : {:?}", ton);
		info!("bitfinity ckbtc : {:?}", bitfinity);
		info!("core ckbtc : {:?}", core);
		info!("osmosis ckbtc : {:?}", osmosis);

		let e_amount = osmosis.parse::<u128>().unwrap_or_default()
			+ bitfinity.parse::<u128>().unwrap_or_default()
			+ ton.parse::<u128>().unwrap_or_default()
			+ core.parse::<u128>().unwrap_or_default();

		info!("ckBTC e_chain_amount: {:?}", &e_amount);
		info!("ckBTC s_chain_amountt: {:?}", &ckbtc_amount);
		info!("ckBTC hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKBTC".to_string(),
			8_i16,
			e_amount.to_string(),
			ckbtc_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && ckbtc_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, ckbtc_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("ckbtc difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_ckusdt(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("CKUSDT_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKUSDT canister ledgers... ");

		let ckusdt_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&ckusdt_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let ckusdt_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckUSDT".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0xe613EBD1eAe99D824Da8A6C33eC833A62bC04B5a").await?;
		info!("bitfinity ckusdt : {:?}", bitfinity);

		let e_amount = bitfinity.parse::<u128>().unwrap_or_default();

		info!("ckUSDT e_chain_amount: {:?}", &e_amount);
		info!("ckUSDT s_chain_amountt: {:?}", &ckusdt_amount);
		info!("ckUSDT hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKUSDT".to_string(),
			6_i16,
			e_amount.to_string(),
			ckusdt_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && ckusdt_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, ckusdt_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("ckusdt difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_neuron_icp(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("NEURON_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on NEURON canister ledgers... ");

		let nicp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&nicp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let nicp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-nICP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x2a78A5f819393105a54F21AdeB4a8b68C5030b02").await?;
		info!("bitfinity nICP : {:?}", bitfinity);

		let e_amount = bitfinity.parse::<u128>().unwrap_or_default();

		info!("nICP e_chain_amount: {:?}", &e_amount);
		info!("nICP s_chain_amountt: {:?}", &nicp_amount);
		info!("nICP hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"neuron ICP".to_string(),
			8_i16,
			e_amount.to_string(),
			nicp_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && nicp_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, nicp_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("nicp difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_dragginz(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("DRAGGIN_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on NEURON canister ledgers... ");

		let nicp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&nicp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let dkp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-DKP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x6286e8464E2817818EF8c3353e91824f680354d2").await?;
		info!("bitfinity dkp : {:?}", bitfinity);

		let e_amount = bitfinity.parse::<u128>().unwrap_or_default();

		info!("dkp e_chain_amount: {:?}", &e_amount);
		info!("dkp s_chain_amountt: {:?}", &dkp_amount);
		info!("dkp hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"Draggin Karma Points".to_string(),
			8_i16,
			e_amount.to_string(),
			dkp_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && dkp_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, dkp_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("dkp difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_icp(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("ICP_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on ICP canister ledgers... ");

		let icp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&icp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let icp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-native-ICP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let osmosis = sync_with_osmosis(
			"factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-native-ICP",
		)
		.await?;
		let bitfinity = sync_with_bitfinity("0x51cCdE9Ca75d95BB55eCe1775fCBFF91324B18A6").await?;
		let ethereum = sync_with_ethereum(
			"0x8e6e7cd8db9c9b73c6c6221702146840b12d6763",
			"275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P",
		)
		.await?;
		let ton = sync_with_ton("EQCW0ddLCQAn011bb8T2Xdoa40v6A_bL3cfjn0bplXdSKnWa").await?;
		let sui = sync_with_sui("0x1c437c7a6acc30d1e1249dbc0bc53dc6f5e1803261bd176d88dec25bc8548af3::icp::ICP")
			.await?
			.parse::<f32>()
			.unwrap_or_default()
			* 100_000_000.0;
		info!("ton icp : {:?}", ton);
		info!("bitfinity icp : {:?}", bitfinity);
		info!("ethereum icp : {:?}", ethereum);
		info!("osmosis icp : {:?}", osmosis);
		info!("sui icp : {:?}", sui);

		let e_amount = osmosis.parse::<u128>().unwrap_or_default()
			+ bitfinity.parse::<u128>().unwrap_or_default()
			+ ethereum.parse::<u128>().unwrap_or_default()
			+ ton.parse::<u128>().unwrap_or_default()
			+ sui as u128;

		info!("ICP e_chain_amount: {:?}", &e_amount);
		info!("ICP s_chain_amountt: {:?}", &icp_amount);
		info!("ICP hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"ICP".to_string(),
			8_i16,
			e_amount.to_string(),
			icp_amount.clone(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;
		if e_amount != 0 && icp_amount.parse::<u128>().unwrap_or(0) != 0 && hub_amount != 0 {
			if difference_warning(e_amount, icp_amount.parse::<u128>().unwrap_or(0), hub_amount) {
				warn!("dkp difference is greater than 1%");
			}
		}

		Ok(())
	})
	.await
}

pub async fn sync_rich(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("EICP_HOPE_YOU_GET_RICH", |agent, canister_id| async move {
		info!("syncing tokens on HOPE_YOU_GET_RICH canister ledgers... ");

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent
			.query(&canister_id, "icrc1_total_supply")
			.with_arg(arg)
			.call()
			.await?;
		let eicp = Decode!(&ret, Nat)?.to_string().replace("_", "");
		let bitfinity = sync_with_bitfinity("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
		let ailayer = sync_with_ailayer("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
		let bitlayer = sync_with_bitlayer("0xb32b737817ba8ff81c696ca8fbd4832cca5751a6").await?;
		let bsquared = sync_with_bsquared("0x20dD93ad6675E81a635C7be034dC1C9Ce0AE2DE4").await?;
		let bevm = sync_with_bevm("0xB76fD1B6CDA18a8cFA255E23059c0bB1624bB5F9").await?;
		let bob = sync_with_bob("0x8f9568BB47b7772f334CcceF4652C9ac7678f21a").await?;
		let ethereum = sync_with_ethereum(
			"0xD14fAd0Fe8175aFD3f4c22B25736E11CF42341A5",
			"275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P",
		)
		.await?;
		let ton = sync_with_ton("EQBGKSkJ307rZY46kqSwwmHskOwSPEO5urm5EZ_EWFyk3bEO").await?;
		let solana = sync_with_solana("8j45TBhQU6DQhRvoYd9dpQWzTNKstB6kpnfZ3pKDCxff").await?;
		let rootstock = sync_with_eth_call(
			"0xb943b047473218a8e0fc637e96136071ffa3f842",
			"https://rootstock-mainnet.g.alchemy.com/v2/cGLTsIuYp7tGOPwDypL0bvmbpjiQQiSp",
		)
		.await?;
		let xlayer =
			sync_with_eth_call("0x51ccde9ca75d95bb55ece1775fcbff91324b18a6", "https://xlayer.drpc.org").await?;
		let merlin = sync_with_eth_call(
			"0xfd4de66eca49799bdde66eb33654e2198ab7bba4",
			"https://rpc.merlinchain.io",
		)
		.await?;
		let core = sync_with_core(
			"0xfd4de66eca49799bdde66eb33654e2198ab7bba4",
			"9ede2feeb2404baabaa4254590950ec6",
		)
		.await?;

		info!("solana Rich : {:?}", solana);
		info!("bob Rich : {:?}", bob);
		info!("rootstock Rich : {:?}", rootstock);
		info!("ethereum Rich : {:?}", ethereum);
		info!("bevm Rich : {:?}", bevm);
		info!("xlayer Rich : {:?}", xlayer);
		info!("merlin Rich : {:?}", merlin);
		info!("ailayer Rich : {:?}", ailayer);
		info!("eicp Rich : {:?}", eicp);
		info!("bitfinity Rich : {:?}", bitfinity);
		info!("bsquared Rich : {:?}", bsquared);
		info!("ton Rich : {:?}", ton);
		info!("bitlayer Rich : {:?}", bitlayer);
		info!("core Rich : {:?}", core);

		let e_amount = eicp.parse::<u128>().unwrap_or_default()
			+ bitfinity.parse::<u128>().unwrap_or_default()
			+ ailayer.parse::<u128>().unwrap_or_default()
			+ bitlayer.parse::<u128>().unwrap_or_default()
			+ bsquared.parse::<u128>().unwrap_or_default()
			+ bevm.parse::<u128>().unwrap_or_default()
			+ bob.parse::<u128>().unwrap_or_default()
			+ ethereum.parse::<u128>().unwrap_or_default()
			+ ton.parse::<u128>().unwrap_or_default()
			+ solana.parse::<u128>().unwrap_or_default()
			+ rootstock.parse::<u128>().unwrap_or_default()
			+ xlayer.parse::<u128>().unwrap_or_default()
			+ merlin.parse::<u128>().unwrap_or_default()
			+ core.parse::<u128>().unwrap_or_default();

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "Bitcoin-runes-HOPE•YOU•GET•RICH".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		info!("RICH e_chain_amount: {:?}", &e_amount);
		info!("RICH s_chain_amountt: {:?}", 0);
		info!("RICH hub_amount: {:?}", &hub_amount);

		let token_on_ledger = token_on_ledger::Model::new(
			"RUNES".to_string(),
			"HOPE•YOU•GET•RICH".to_string(),
			2_i16,
			e_amount.to_string(),
			"0".to_string(),
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		if e_amount.ge(&hub_amount) {
			if (hub_amount as f64) / (e_amount as f64) < 0.99 {
				warn!("Rich difference is greater than 1%");
			}
		} else {
			if (hub_amount as f64) / (e_amount as f64) < 0.99 {
				warn!("Rich difference is greater than 1%");
			}
		}
		Ok(())
	})
	.await
}

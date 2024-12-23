use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, U256};

#[derive(Serialize, Deserialize)]
struct RpcRequest {
	id: u32,
	jsonrpc: String,
	method: String,
	params: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct EthcallRpcRequest {
	id: u32,
	jsonrpc: String,
	method: String,
	params: Vec<serde_json::Value>,
}

pub async fn sync_with_eth_call(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let method_signature = "totalSupply()";
	let method_hash = web3::signing::keccak256(method_signature.as_bytes());
	let data = Bytes::from(method_hash);

	let params = vec![
		serde_json::json!({
			"to": ledger_id,
			"data": data,
		}),
		serde_json::json!("latest"),
	];

	let rpc_request = EthcallRpcRequest {
		id: 1,
		jsonrpc: "2.0".to_string(),
		method: "eth_call".to_string(),
		params,
	};

	let client = reqwest::Client::new();

	let response = client
		.post("https://rootstock-mainnet.g.alchemy.com/v2/cGLTsIuYp7tGOPwDypL0bvmbpjiQQiSp")
		.header("accept", "application/json")
		.header("content-Type", "application/json")
		.json(&rpc_request)
		.send()
		.await?;

	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(result) = layer_one.get("result") {
				let mut amount = result.to_string();
				amount.replace_range(0..1, "");
				amount.replace_range((amount.len() - 1).., "");
				let raw_data = amount.as_str();
				let value = U256::from_str_radix(raw_data.trim_start_matches("0x"), 16).unwrap();
				println!("{:?}", value.to_string());
				return Ok(value.to_string());
			} else {
				return Err("eth call error1".into());
			}
		} else {
			return Err("eth call error2".into());
		}
	} else {
		return Err("eth call error3".into());
	}
}

//sync_with_solana("8j45TBhQU6DQhRvoYd9dpQWzTNKstB6kpnfZ3pKDCxff").await?;
pub async fn sync_with_solana(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let rpc_request = RpcRequest {
		id: 1,
		jsonrpc: "2.0".to_string(),
		method: "getTokenSupply".to_string(),
		params: vec![ledger_id.to_string()],
	};

	let client = reqwest::Client::new();

	let response = client
		.post("https://solana-mainnet.g.alchemy.com/v2/cGLTsIuYp7tGOPwDypL0bvmbpjiQQiSp")
		.header("accept", "application/json")
		.header("content-Type", "application/json")
		.json(&rpc_request)
		.send()
		.await?;

	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(layer_two) = layer_one.get("result") {
				if let Some(layer_three) = layer_two.as_object() {
					if let Some(layer_four) = layer_three.get("value") {
						if let Some(layer_five) = layer_four.as_object() {
							if let Some(layer_six) = layer_five.get("amount") {
								let mut amount = layer_six.to_string();
								amount.replace_range(0..1, "");
								amount.replace_range((amount.len() - 1).., "");
								println!("{:?}", amount);
								return Ok(amount);
							} else {
								return Err("solana error7".into());
							}
						} else {
							return Err("solana error6".into());
						}
					} else {
						return Err("solana error5".into());
					}
				} else {
					return Err("solana error4".into());
				}
			} else {
				return Err("solana error3".into());
			}
		} else {
			return Err("solana error2".into());
		}
	} else {
		return Err("solana error1".into());
	}
}

//sync_with_core("0xfd4de66eca49799bdde66eb33654e2198ab7bba4","9ede2feeb2404baabaa4254590950ec6").
// await?;
pub async fn sync_with_core(ledger_id: &str, api_token: &str) -> Result<String, Box<dyn Error>> {
	let transport = web3::transports::Http::new("https://api.zan.top/core-mainnet")?;
	let web3 = web3::Web3::new(transport);
	let url = "https://openapi.coredao.org/api?module=contract&action=getabi&address=".to_string()
		+ ledger_id
		+ "&apikey="
		+ api_token;

	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;

	if let Ok(vjson) = serde_json::from_str::<serde_json::Value>(&body) {
		let abi = match vjson["result"].as_str() {
			Some(abi_str) => abi_str,
			None => {
				return Err("Error: Unable to fetch ABI".into());
			}
		};
		let contract_address = Address::from_str(ledger_id)?;
		let contract = Contract::from_json(web3.eth(), contract_address, abi.as_bytes())?;
		let result: U256 = contract
			.query("totalSupply", (), None, Options::default(), None)
			.await?;
		println!("Total supply: {}", result);
		return Ok(result.to_string());
	} else {
		return Err("core error".into());
	}
}

pub async fn sync_with_osmosis(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let client = reqwest::Client::new();
	let url = "https://osmosis-rest.publicnode.com/osmosis/superfluid/v1beta1/supply?denom=".to_string() + ledger_id;
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(layer_two) = layer_one.get("amount") {
				if let Some(layer_three) = layer_two.as_object() {
					if let Some(layer_four) = layer_three.get("amount") {
						let mut amount = layer_four.to_string();
						amount.replace_range(0..1, "");
						amount.replace_range((amount.len() - 1).., "");
						return Ok(amount);
					} else {
						return Err("osmosis error1".into());
					}
				} else {
					return Err("osmosis error2".into());
				}
			} else {
				return Err("osmosis error3".into());
			}
		} else {
			return Err("osmosis error4".into());
		}
	} else {
		return Err("osmosis error5".into());
	}
}

pub async fn sync_with_ton(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://toncenter.com/api/v2/getTokenData?address=".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;

	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(layer_two) = layer_one.get("result") {
				if let Some(layer_there) = layer_two.as_object() {
					if let Some(total_supply) = layer_there.get("total_supply") {
						return Ok(total_supply.to_string());
					} else {
						return Err("ton error1".into());
					}
				} else {
					return Err("ton error2".into());
				}
			} else {
				return Err("ton error3".into());
			}
		} else {
			return Err("ton error4".into());
		}
	} else {
		return Err("ton error5".into());
	}
}

pub async fn sync_with_ethereum(ledger_id: &str, api_token: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=tokensupply&contractaddress=".to_string()
		+ ledger_id
		+ "&apikey="
		+ api_token;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("result") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("ethereum error1".into());
			}
		} else {
			return Err("ethereum error2".into());
		}
	} else {
		return Err("ethereum error3".into());
	}
}

pub async fn sync_with_bitfinity(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://explorer.mainnet.bitfinity.network/api/v2/tokens/".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let bitfinity_response = client.get(url).send().await?;
	let bitfinity_body = bitfinity_response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&bitfinity_body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("total_supply") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("bitfinity error1".into());
			}
		} else {
			return Err("bitfinity error2".into());
		}
	} else {
		return Err("bitfinity error3".into());
	}
}

pub async fn sync_with_ailayer(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://mainnet-explorer.ailayer.xyz/api/v2/tokens/".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("total_supply") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("ai layer error1".into());
			}
		} else {
			return Err("ai layer error2".into());
		}
	} else {
		return Err("ai layer error3".into());
	}
}

pub async fn sync_with_bitlayer(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url =
		"https://api.btrscan.com/scan/api?module=token&action=tokensupply&contractaddress=".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;

	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("result") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("bitlayer error1".into());
			}
		} else {
			return Err("bitlayer error2".into());
		}
	} else {
		return Err("bitlayer error3".into());
	}
}

pub async fn sync_with_bsquared(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://explorer.bsquared.network/api?contractaddress=".to_string()
		+ ledger_id
		+ "&module=token&action=tokeninfo";
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(layer_two) = layer_one.get("result") {
				if let Some(layer_three) = layer_two.as_array() {
					if let Some(layer_four) = layer_three[0].as_object() {
						if let Some(layer_five) = layer_four.get("totalSupply") {
							let mut amount = layer_five.to_string();
							amount.replace_range(0..1, "");
							amount.replace_range((amount.len() - 1).., "");
							return Ok(amount);
						} else {
							return Err("bsquared5 error".into());
						}
					} else {
						return Err("bsquared4 error".into());
					}
				} else {
					return Err("bsquared3 error".into());
				}
			} else {
				return Err("bsquared2 error".into());
			}
		} else {
			return Err("bsquared1 error".into());
		}
	} else {
		return Err("bsquared0 error".into());
	}
}

pub async fn sync_with_bevm(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://scan-mainnet-api.bevm.io/api/v2/tokens/".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("total_supply") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("bevm error1".into());
			}
		} else {
			return Err("bevm error2".into());
		}
	} else {
		return Err("bevm error3".into());
	}
}

pub async fn sync_with_bob(ledger_id: &str) -> Result<String, Box<dyn Error>> {
	let url = "https://explorer.gobob.xyz/api/v2/tokens/".to_string() + ledger_id;
	let client = reqwest::Client::new();
	let response = client.get(url).send().await?;
	let body = response.text().await?;
	if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
		if let Some(layer_one) = value.as_object() {
			if let Some(ttl) = layer_one.get("total_supply") {
				let mut total_supply = ttl.to_string();
				total_supply.replace_range(0..1, "");
				total_supply.replace_range((total_supply.len() - 1).., "");
				return Ok(total_supply);
			} else {
				return Err("bob error1".into());
			}
		} else {
			return Err("bob error2".into());
		}
	} else {
		return Err("bob error3".into());
	}
}

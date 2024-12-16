use reqwest;
use std::error::Error;

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
						return Err("osmosis error".into());
					}
				} else {
					return Err("osmosis error".into());
				}
			} else {
				return Err("osmosis error".into());
			}
		} else {
			return Err("osmosis error".into());
		}
	} else {
		return Err("osmosis error".into());
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
				return Err("bitfinity error".into());
			}
		} else {
			return Err("bitfinity error".into());
		}
	} else {
		return Err("bitfinity error".into());
	}
}

// sync_with_ailayer("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
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
				return Err("ai layer error".into());
			}
		} else {
			return Err("ai layer error".into());
		}
	} else {
		return Err("ai layer error".into());
	}
}

//sync_with_bitlayer("0xb32b737817ba8ff81c696ca8fbd4832cca5751a6").await?;
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
				return Err("bitlayer error".into());
			}
		} else {
			return Err("bitlayer error".into());
		}
	} else {
		return Err("bitlayer error".into());
	}
}

// sync_with_bsquared("0x20dd93ad6675e81a635c7be034dc1c9ce0ae2de4").await?;
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

// sync_with_bevm("0xB76fD1B6CDA18a8cFA255E23059c0bB1624bB5F9").await?;
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
				return Err("bevm error".into());
			}
		} else {
			return Err("bevm error".into());
		}
	} else {
		return Err("bevm error".into());
	}
}

// sync_with_ton("EQCW0ddLCQAn011bb8T2Xdoa40v6A_bL3cfjn0bplXdSKnWa").await?;
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
						return Err("ton error".into());
					}
				} else {
					return Err("ton error".into());
				}
			} else {
				return Err("ton error".into());
			}
		} else {
			return Err("ton error".into());
		}
	} else {
		return Err("ton error".into());
	}
}

// sync_with_bob("0x8f9568BB47b7772f334CcceF4652C9ac7678f21a").await?;
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
				return Err("bob error".into());
			}
		} else {
			return Err("bob error".into());
		}
	} else {
		return Err("bob error".into());
	}
}

//sync_with_ethereum("0xD14fAd0Fe8175aFD3f4c22B25736E11CF42341A5&
// apikey=275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
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
				return Err("ethereum error".into());
			}
		} else {
			return Err("ethereum error".into());
		}
	} else {
		return Err("ethereum error".into());
	}
}

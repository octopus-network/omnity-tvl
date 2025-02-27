use anyhow::anyhow;
use ic_agent::{export::Principal, identity::Secp256k1Identity, Agent, Identity};
use log::info;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;
use std::{error::Error, future::Future, sync::Arc};

pub struct Database {
	pub connection: Arc<DatabaseConnection>,
}

impl Database {
	pub async fn new(db_url: String) -> Self {
		let mut opt = ConnectOptions::new(db_url);
		opt.max_connections(100)
			.min_connections(5)
			.connect_timeout(Duration::from_secs(8))
			.acquire_timeout(Duration::from_secs(8))
			.idle_timeout(Duration::from_secs(8))
			.max_lifetime(Duration::from_secs(8))
			.sqlx_logging(false)
			.sqlx_logging_level(log::LevelFilter::Info);

		let connection = sea_orm::Database::connect(opt)
			.await
			.expect("Could not connect to database");
		assert!(connection.ping().await.is_ok());
		info!("Connected to database !");

		Database {
			connection: Arc::new(connection),
		}
	}

	pub fn get_connection(&self) -> Arc<DatabaseConnection> {
		self.connection.clone()
	}
}

pub async fn create_agent(identity: impl Identity + 'static) -> Result<Agent, String> {
	let network = std::env::var("DFX_NETWORK")
		.map_err(|_| anyhow!("DFX_NETWORK is not found"))
		.unwrap();

	Agent::builder()
		.with_url(network)
		.with_identity(identity)
		.build()
		.map_err(|e| format!("{:?}", e))
}

pub async fn with_agent_as<I, F, R>(agent_identity: I, f: F) -> Result<(), Box<dyn Error>>
where
	I: Identity + 'static,
	R: Future<Output = Result<(), Box<dyn Error>>>,
	F: FnOnce(Agent) -> R,
{
	let agent = create_agent(agent_identity).await?;
	f(agent).await
}

pub async fn with_agent<F, R>(f: F) -> Result<(), Box<dyn Error>>
where
	R: Future<Output = Result<(), Box<dyn Error>>>,
	F: FnOnce(Agent) -> R,
{
	let identity = std::env::var("DFX_IDENTITY")
		.map_err(|_| anyhow!("DFX_IDENTITY is not found"))
		.unwrap();
	let agent_identity = Secp256k1Identity::from_pem(identity.as_bytes())?;

	with_agent_as(agent_identity, f).await?;
	Ok(())
}

pub async fn with_canister<F, R>(canister: &str, f: F) -> Result<(), Box<dyn Error>>
where
	R: Future<Output = Result<(), Box<dyn Error>>>,
	F: FnOnce(Agent, Principal) -> R,
{
	with_agent(|agent| async move {
		let canister_id = Principal::from_text(std::env::var(canister)?)?;
		f(agent, canister_id).await
	})
	.await
}

pub fn difference_warning(a: u128, b: u128, c: u128) -> bool {
	let numbers = vec![a, b, c];
	let mut sorted_numbers = numbers.clone();
	sorted_numbers.sort();

	if (sorted_numbers[0] as f64) / (sorted_numbers[1] as f64) < 0.99 {
		return true;
	} else if (sorted_numbers[0] as f64) / (sorted_numbers[2] as f64) < 0.99 {
		return true;
	} else if (sorted_numbers[1] as f64) / (sorted_numbers[2] as f64) < 0.99 {
		return true;
	}
	false
}

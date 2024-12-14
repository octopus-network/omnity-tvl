use anyhow::anyhow;
use dotenvy::dotenv;
// use log::LevelFilter;
// use log4rs::{
// 	append::console::ConsoleAppender,
// 	config::{Appender, Root},
// };
use std::error::Error;
use tvl::{tasks::execute_sync_tasks, utils::Database};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
	dotenv().ok();

	// let stdout = ConsoleAppender::builder().build();
	// let config = log4rs::config::Config::builder()
	// 	.appender(Appender::builder().build("stdout", Box::new(stdout)))
	// 	.build(Root::builder().appender("stdout").build(LevelFilter::Info))
	// 	.unwrap();
	// log4rs::init_config(config).unwrap();

	if let Err(e) = log4rs::init_file("./log4rs.yaml", Default::default()) {
		eprintln!("init log failed: {}", e);
		std::process::exit(1);
	}

	let db_url = std::env::var("DATABASE_URL").map_err(|_| anyhow!("DATABASE_URL is not found"))?;

	let db = Database::new(db_url.clone()).await;

	execute_sync_tasks(db.get_connection()).await;

	Ok(())
}

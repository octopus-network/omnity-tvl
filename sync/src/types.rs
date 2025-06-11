use crate::entities::token_on_ledger;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type ChainId = String;
pub type TokenId = String;

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum Error {
	#[error("The chain(`{0}`) already exists")]
	ChainAlreadyExisting(String),
	#[error("The token(`{0}`) already exists")]
	TokenAlreadyExisting(String),
	#[error("not supported proposal")]
	NotSupportedProposal,
	#[error("proposal error: (`{0}`)")]
	ProposalError(String),
	#[error("generate directive error for : (`{0}`)")]
	GenerateDirectiveError(String),
	#[error("the message is malformed and cannot be decoded error")]
	MalformedMessageBytes,
	#[error("unauthorized")]
	Unauthorized,
	#[error("The `{0}` is deactive")]
	DeactiveChain(String),
	#[error("The ticket id (`{0}`) already exists!")]
	AlreadyExistingTicketId(String),
	#[error("The resubmit ticket id must exist!")]
	ResubmitTicketIdMustExist,
	#[error("The resubmit ticket must same as the old ticket!")]
	ResubmitTicketMustSame,
	#[error("The resumit ticket sent too often")]
	ResubmitTicketSentTooOften,
	#[error("not found chain: (`{0}`)")]
	NotFoundChain(String),
	#[error("not found token: (`{0}`)")]
	NotFoundToken(String),
	#[error("not found account(`{0}`) token(`{1}`) on the chain(`{2}`")]
	NotFoundAccountToken(String, String, String),
	#[error("Not found this token(`{0}`) on chain(`{1}`) ")]
	NotFoundChainToken(String, String),
	#[error("Insufficient token (`{0}`) on chain (`{1}`) !)")]
	NotSufficientTokens(String, String),
	#[error("The ticket amount(`{0}`) parse error: `{1}`")]
	TicketAmountParseError(String, String),
	#[error("ecdsa_public_key failed : (`{0}`)")]
	EcdsaPublicKeyError(String),
	#[error("sign_with_ecdsa failed: (`{0}`)")]
	SighWithEcdsaError(String),
	#[error("custom error: (`{0}`)")]
	CustomError(String),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OmnityTokenOnChain {
	pub chain_id: ChainId,
	pub token_id: TokenId,
	pub amount: u128,
}

impl token_on_ledger::Model {
	pub fn new(
		chain_id: String,
		token_id: String,
		decimals: i16,
		e_chain_amount: String,
		s_chain_amount: String,
		hub_amount: String,
	) -> Self {
		token_on_ledger::Model {
			chain_id,
			token_id,
			decimals,
			e_chain_amount,
			s_chain_amount,
			hub_amount,
		}
	}
}

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
// pub struct Chain {
// 	pub chain_id: ChainId,
// 	pub canister_id: String,
// 	pub chain_type: ChainType,
// 	pub chain_state: ChainState,
// 	pub contract_address: Option<String>,
// 	pub counterparties: Option<Vec<ChainId>>,
// 	pub fee_token: Option<TokenId>,
// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum ChainType {
// 	SettlementChain,
// 	ExecutionChain,
// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum ChainState {
// 	Active,
// 	Deactive,
// }

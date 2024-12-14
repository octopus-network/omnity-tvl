use crate::entities::{token_on_chain, token_on_ledger};
use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use serde::{Deserialize, Serialize};
use std::fmt;
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

/// Represents an error from a management canister call, such as
/// `sign_with_ecdsa` or `bitcoin_send_transaction`.
#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct CallError {
	pub method: String,
	pub reason: Reason,
}

impl fmt::Display for CallError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "management call '{}' failed: {}", self.method, self.reason)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
/// The reason for the management call failure.
pub enum Reason {
	/// Failed to send a signature request because the local output queue is
	/// full.
	QueueIsFull,
	/// The canister does not have enough cycles to submit the request.
	OutOfCycles,
	/// The call failed with an error.
	CanisterError(String),
	/// The management canister rejected the signature request (not enough
	/// cycles, the ECDSA subnet is overloaded, etc.).
	Rejected(String),
}

impl fmt::Display for Reason {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::QueueIsFull => write!(fmt, "the canister queue is full"),
			Self::OutOfCycles => write!(fmt, "the canister is out of cycles"),
			Self::CanisterError(msg) => write!(fmt, "canister error: {}", msg),
			Self::Rejected(msg) => {
				write!(fmt, "the management canister rejected the call: {}", msg)
			}
		}
	}
}

impl Reason {
	pub fn from_reject(reject_code: RejectionCode, reject_message: String) -> Self {
		match reject_code {
			RejectionCode::CanisterReject => Self::Rejected(reject_message),
			_ => Self::CanisterError(reject_message),
		}
	}
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OmnityTokenOnChain {
	// the chain of the token be locked
	pub chain_id: ChainId,
	pub token_id: TokenId,
	pub amount: u128,
}

impl core::fmt::Display for OmnityTokenOnChain {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(
			f,
			"\nchain id:{} \ntoken id:{}  \namount:{} ",
			self.chain_id, self.token_id, self.amount
		)
	}
}

impl From<OmnityTokenOnChain> for token_on_chain::Model {
	fn from(token_on_chain: OmnityTokenOnChain) -> Self {
		token_on_chain::Model {
			chain_id: token_on_chain.chain_id,
			token_id: token_on_chain.token_id,
			amount: token_on_chain.amount.to_string(),
		}
	}
}
impl From<token_on_chain::Model> for OmnityTokenOnChain {
	fn from(model: token_on_chain::Model) -> Self {
		OmnityTokenOnChain {
			chain_id: model.chain_id,
			token_id: model.token_id,
			amount: model.amount.parse::<u128>().unwrap(),
		}
	}
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

pub use ethereum::{AccessList, AccessListItem, Account};
pub use evm::{backend::Log, Config, ExitError, ExitFatal, ExitReason, ExitRevert, ExitSucceed};

use super::MIN_GAS_PRICE;
use crate::codec::ProtocolCodec;
use crate::types::{Hash, Hasher, Header, MerkleRoot, Proposal, H160, U256};
use rlp_derive::{RlpDecodable, RlpEncodable};

pub const WORLD_STATE_META_KEY: [u8; 1] = [0];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecResp {
    pub state_root: MerkleRoot,
    pub transaction_root: MerkleRoot,
    pub receipt_root: MerkleRoot,
    pub gas_used: u64,
    pub fee_used: U256, // sum(<gas in tx * gas price setted by tx> ...)
    pub txs_resp: Vec<TxResp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxResp {
    pub exit_reason: ExitReason,
    pub ret: Vec<u8>,
    pub gas_used: u64,
    pub remain_gas: u64,
    pub fee_cost: U256,
    pub logs: Vec<Log>,
    pub code_address: Option<Hash>,
    pub removed: bool,
}

impl TxResp {
    pub fn invalid_nonce(gas_used: u64, fee_cost: U256) -> Self {
        TxResp {
            exit_reason: ExitReason::Error(ExitError::Other("invalid nonce".into())),
            gas_used,
            remain_gas: u64::default(),
            fee_cost,
            removed: false,
            ret: vec![],
            logs: vec![],
            code_address: None,
        }
    }
}

impl Default for TxResp {
    fn default() -> Self {
        TxResp {
            exit_reason: ExitReason::Succeed(ExitSucceed::Stopped),
            gas_used: u64::default(),
            remain_gas: u64::default(),
            fee_cost: U256::default(),
            removed: false,
            ret: vec![],
            logs: vec![],
            code_address: None,
        }
    }
}

#[derive(RlpEncodable, RlpDecodable, Default, Clone, Debug, PartialEq, Eq)]
pub struct ExecutorContext {
    pub block_number: U256,
    pub block_hash: Hash,
    pub block_coinbase: H160,
    pub block_timestamp: U256,
    pub chain_id: U256,
    pub difficulty: U256,
    pub origin: H160,
    pub gas_price: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
    pub logs: Vec<Log>,
}

impl From<&Proposal> for ExecutorContext {
    fn from(p: &Proposal) -> Self {
        let gas_price = U256::from(MIN_GAS_PRICE);
        ExecutorContext {
            block_number: p.number.into(),
            block_hash: Hasher::digest(p.encode().unwrap()),
            block_coinbase: p.proposer,
            block_timestamp: p.timestamp.into(),
            chain_id: p.chain_id.into(),
            difficulty: U256::one(),
            origin: p.proposer,
            gas_price,
            block_gas_limit: p.gas_limit,
            block_base_fee_per_gas: p.base_fee_per_gas,
            logs: Vec::new(),
        }
    }
}

impl From<&Header> for ExecutorContext {
    fn from(h: &Header) -> ExecutorContext {
        let gas_price = U256::from(MIN_GAS_PRICE);
        ExecutorContext {
            block_number: h.number.into(),
            block_hash: Hasher::digest(h.encode().unwrap()),
            block_coinbase: h.proposer,
            block_timestamp: h.timestamp.into(),
            chain_id: h.chain_id.into(),
            difficulty: U256::one(),
            origin: h.proposer,
            gas_price,
            block_gas_limit: h.gas_limit,
            block_base_fee_per_gas: h.base_fee_per_gas,
            logs: Vec::new(),
        }
    }
}

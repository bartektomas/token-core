use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxSignResult {
    #[serde(rename = "sign")]
    pub signature: String,
    #[serde(rename = "hash")]
    pub tx_hash: String,
    pub wtx_id: String,
}

impl SignedTransaction for TxSignResult {}

pub trait Transaction: Sized {}

pub trait SignedTransaction: Sized {}

pub trait TransactionSigner<Input: Transaction, Output: SignedTransaction> {
    fn sign_transaction(&self, tx: &Input, password: Option<&str>) -> Result<Output>;
}

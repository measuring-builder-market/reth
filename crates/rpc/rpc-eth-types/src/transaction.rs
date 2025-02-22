//! Helper types for `reth_rpc_eth_api::EthApiServer` implementation.
//!
//! Transaction wrapper that labels transaction with its origin.
use reth_primitives::{TransactionSignedEcRecovered, B256};
use reth_rpc_types::{Transaction, TransactionInfo, WithOtherFields};
use reth_rpc_types_compat::transaction::from_recovered_with_block_context;

/// Represents from where a transaction was fetched.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionSource {
    /// Transaction exists in the pool (Pending)
    Pool(TransactionSignedEcRecovered),
    /// Transaction already included in a block
    ///
    /// This can be a historical block or a pending block (received from the CL)
    Block {
        /// Transaction fetched via provider
        transaction: TransactionSignedEcRecovered,
        /// Index of the transaction in the block
        index: u64,
        /// Hash of the block.
        block_hash: B256,
        /// Number of the block.
        block_number: u64,
        /// base fee of the block.
        base_fee: Option<u64>,
    },
}

// === impl TransactionSource ===

impl TransactionSource {
    /// Consumes the type and returns the wrapped transaction.
    pub fn into_recovered(self) -> TransactionSignedEcRecovered {
        self.into()
    }

    /// Returns the transaction and block related info, if not pending
    pub fn split(self) -> (TransactionSignedEcRecovered, TransactionInfo) {
        match self {
            Self::Pool(tx) => {
                let hash = tx.hash();
                (
                    tx,
                    TransactionInfo {
                        hash: Some(hash),
                        index: None,
                        block_hash: None,
                        block_number: None,
                        base_fee: None,
                    },
                )
            }
            Self::Block { transaction, index, block_hash, block_number, base_fee } => {
                let hash = transaction.hash();
                (
                    transaction,
                    TransactionInfo {
                        hash: Some(hash),
                        index: Some(index),
                        block_hash: Some(block_hash),
                        block_number: Some(block_number),
                        base_fee: base_fee.map(u128::from),
                    },
                )
            }
        }
    }
}

impl From<TransactionSource> for TransactionSignedEcRecovered {
    fn from(value: TransactionSource) -> Self {
        match value {
            TransactionSource::Pool(tx) => tx,
            TransactionSource::Block { transaction, .. } => transaction,
        }
    }
}

impl From<TransactionSource> for WithOtherFields<Transaction> {
    fn from(value: TransactionSource) -> Self {
        match value {
            TransactionSource::Pool(tx) => reth_rpc_types_compat::transaction::from_recovered(tx),
            TransactionSource::Block { transaction, index, block_hash, block_number, base_fee } => {
                let tx_info = TransactionInfo {
                    hash: Some(transaction.hash()),
                    block_hash: Some(block_hash),
                    block_number: Some(block_number),
                    base_fee: base_fee.map(u128::from),
                    index: Some(index),
                };
                from_recovered_with_block_context(transaction, tx_info)
            }
        }
    }
}

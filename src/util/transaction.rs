#[cfg(any(feature = "ocean", feature = "liquid"))]
use bitcoin::hashes::hex::FromHex;
use bitcoin::hashes::sha256d::Hash as Sha256dHash;

use crate::chain::{TxIn, TxOut};
use crate::util::BlockId;

#[cfg(any(feature = "ocean", feature = "liquid"))]
lazy_static! {
    static ref INITIAL_ISSUANCE_PREVOUT: Sha256dHash =
        Sha256dHash::from_hex("4b663b0279c27f3f7b1abf81a0c847e264594274f7bfba984ba9a99544a95515")
            .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct TransactionStatus {
    pub confirmed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<Sha256dHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u32>,
}

impl From<Option<BlockId>> for TransactionStatus {
    fn from(blockid: Option<BlockId>) -> TransactionStatus {
        match blockid {
            Some(b) => TransactionStatus {
                confirmed: true,
                block_height: Some(b.height as usize),
                block_hash: Some(b.hash),
                block_time: Some(b.time),
            },
            None => TransactionStatus {
                confirmed: false,
                block_height: None,
                block_hash: None,
                block_time: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TxInput {
    pub txid: Sha256dHash,
    pub vin: u16,
}

pub fn is_coinbase(txin: &TxIn) -> bool {
    #[cfg(not(any(feature = "ocean", feature = "liquid")))]
    return txin.previous_output.is_null();
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    return txin.is_coinbase();
}

pub fn has_prevout(txin: &TxIn) -> bool {
    #[cfg(not(any(feature = "ocean", feature = "liquid")))]
    return !txin.previous_output.is_null();
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    return !txin.is_coinbase()
        && !txin.is_pegin
        && txin.previous_output.txid != *INITIAL_ISSUANCE_PREVOUT;
}

pub fn is_spendable(txout: &TxOut) -> bool {
    #[cfg(not(any(feature = "ocean", feature = "liquid")))]
    return !txout.script_pubkey.is_provably_unspendable();
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    return !txout.is_fee() && !txout.script_pubkey.is_provably_unspendable();
}

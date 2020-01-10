#[cfg(not(feature = "ocean"))]
pub use bitcoin::util::address;
#[cfg(not(feature = "ocean"))] // use regular Bitcoin data structures
pub use bitcoin::{Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

#[cfg(feature = "ocean")]
pub use elements::address;
#[cfg(feature = "ocean")]
pub use elements::{confidential, Address, Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

use bitcoin::blockdata::constants::genesis_block;
use bitcoin::hashes::sha256d::Hash as Sha256dHash;
use bitcoin::network::constants::Network as BNetwork;
use bitcoin::util::hash::BitcoinHash;

#[cfg(not(feature = "ocean"))]
pub type Value = u64;
#[cfg(feature = "ocean")]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,

    #[cfg(feature = "ocean")]
    Ocean,
    #[cfg(feature = "ocean")]
    Gold,
    #[cfg(feature = "ocean")]
    OceanRegtest,
}

impl Network {
    pub fn genesis_hash(&self) -> Sha256dHash {
        let block = genesis_block(BNetwork::from(self));
        block.bitcoin_hash()
    }

    pub fn magic(&self) -> u32 {
        match self {
            Network::Bitcoin => 0xD9B4BEF9,
            Network::Testnet => 0x0709110B,
            Network::Regtest => 0xDAB5BFFA,

            #[cfg(feature = "ocean")]
            Network::Ocean => 0xDAB5BFFA,
            #[cfg(feature = "ocean")]
            Network::Gold => 0xDAB5BFFA,
            #[cfg(feature = "ocean")]
            Network::OceanRegtest => 0xDAB5BFFA,
        }
    }

    #[cfg(feature = "ocean")]
    pub fn address_params(&self) -> &'static address::AddressParams {
        // ocean regtest uses elements's address params
        match self {
            Network::Ocean => &address::AddressParams::OCEAN,
            Network::Gold => &address::AddressParams::GOLD,
            Network::OceanRegtest => &address::AddressParams::ELEMENTS,
            _ => panic!("the ocean-only address_params() called with non-ocean network"),
        }
    }

    pub fn names() -> Vec<String> {
        #[cfg(not(feature = "ocean"))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
        ];

        #[cfg(feature = "ocean")]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "ocean".to_string(),
            "gold".to_string(),
            "oceanregtest".to_string(),
        ];
    }
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,

            #[cfg(feature = "ocean")]
            "ocean" => Network::Ocean,
            #[cfg(feature = "ocean")]
            "gold" => Network::Gold,
            #[cfg(feature = "ocean")]
            "oceanregtest" => Network::OceanRegtest,

            _ => panic!("unsupported Bitcoin network: {:?}", network_name),
        }
    }
}

impl From<&Network> for BNetwork {
    fn from(network: &Network) -> Self {
        match network {
            Network::Bitcoin => BNetwork::Bitcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,

            #[cfg(feature = "ocean")]
            Network::Ocean => BNetwork::Bitcoin, // @FIXME
            #[cfg(feature = "ocean")]
            Network::Gold => BNetwork::Bitcoin, // @FIXME
            #[cfg(feature = "ocean")]
            Network::OceanRegtest => BNetwork::Regtest, // @FIXME
        }
    }
}

impl From<&BNetwork> for Network {
    fn from(network: &BNetwork) -> Self {
        match network {
            #[cfg(not(feature = "ocean"))]
            BNetwork::Bitcoin => Network::Bitcoin,
            #[cfg(not(feature = "ocean"))]
            BNetwork::Regtest => Network::Regtest,

            #[cfg(feature = "ocean")]
            BNetwork::Bitcoin => Network::Ocean, // @FIXME
            #[cfg(feature = "ocean")]
            BNetwork::Regtest => Network::OceanRegtest, // @FIXME
            BNetwork::Testnet => Network::Testnet, // @FIXME
        }
    }
}

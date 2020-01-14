#[cfg(not(any(feature = "ocean", feature = "liquid")))]
pub use bitcoin::util::address;
#[cfg(not(any(feature = "ocean", feature = "liquid")))] // use regular Bitcoin data structures
pub use bitcoin::{Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

#[cfg(any(feature = "ocean", feature = "liquid"))]
pub use elements::address;
#[cfg(any(feature = "ocean", feature = "liquid"))]
pub use elements::{confidential, Address, Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

use bitcoin::blockdata::constants::genesis_block;
use bitcoin::hashes::sha256d::Hash as Sha256dHash;
use bitcoin::network::constants::Network as BNetwork;
use bitcoin::util::hash::BitcoinHash;

#[cfg(not(any(feature = "ocean", feature = "liquid")))]
pub type Value = u64;
#[cfg(any(feature = "ocean", feature = "liquid"))]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    Bitcoin,
    Testnet,
    Regtest,

    #[cfg(any(feature = "ocean", feature = "liquid"))]
    Ocean,
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    Gold,
    #[cfg(any(feature = "ocean", feature = "liquid"))]
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

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => 0xDAB5BFFA,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => 0xDAB5BFFA,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => 0xDAB5BFFA,
        }
    }

    #[cfg(any(feature = "ocean", feature = "liquid"))]
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
        #[cfg(not(any(feature = "ocean", feature = "liquid")))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
        ];

        #[cfg(any(feature = "ocean", feature = "liquid"))]
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

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            "ocean" => Network::Ocean,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            "gold" => Network::Gold,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
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

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => BNetwork::Bitcoin, // @FIXME
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => BNetwork::Bitcoin, // @FIXME
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => BNetwork::Regtest, // @FIXME
        }
    }
}

impl From<&BNetwork> for Network {
    fn from(network: &BNetwork) -> Self {
        match network {
            #[cfg(not(any(feature = "ocean", feature = "liquid")))]
            BNetwork::Bitcoin => Network::Bitcoin,
            #[cfg(not(any(feature = "ocean", feature = "liquid")))]
            BNetwork::Regtest => Network::Regtest,

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            BNetwork::Bitcoin => Network::Ocean, // @FIXME
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            BNetwork::Regtest => Network::OceanRegtest, // @FIXME
            BNetwork::Testnet => Network::Testnet, // @FIXME
        }
    }
}

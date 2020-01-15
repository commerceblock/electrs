use clap::{App, Arg};
use dirs::home_dir;
use num_cpus;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use stderrlog;
use std::net::ToSocketAddrs;

#[cfg(any(feature = "ocean", feature = "liquid"))]
use bitcoin::hashes::hex::ToHex;

use crate::chain::Network;
use crate::daemon::CookieGetter;

use crate::errors::*;

#[derive(Debug, Clone)]
pub struct Config {
    // See below for the documentation of each field:
    pub log: stderrlog::StdErrLog,
    pub network_type: Network,
    pub db_path: PathBuf,
    pub daemon_dir: PathBuf,
    pub daemon_rpc_addr: SocketAddr,
    pub cookie: Option<String>,
    pub electrum_rpc_addr: SocketAddr,
    pub http_addr: SocketAddr,
    pub monitoring_addr: SocketAddr,
    pub jsonrpc_import: bool,
    pub index_batch_size: usize,
    pub bulk_index_threads: usize,
    pub tx_cache_size: usize,
    pub prevout_enabled: bool,
    pub cors: Option<String>,
    pub precache_scripts: Option<String>,

    #[cfg(any(feature = "ocean", feature = "liquid"))]
    pub parent_network: Network,
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    pub parent_genesis_hash: String,
    #[cfg(any(feature = "ocean", feature = "liquid"))]
    pub asset_db_path: Option<PathBuf>,
}

impl Config {
    pub fn from_args() -> Config {
        let network_help = format!(
            "Select Bitcoin network type ({})",
            Network::names().join(", ")
        );

        let args = App::new("Electrum Rust Server")
            .version(crate_version!())
            .arg(
                Arg::with_name("verbosity")
                    .short("v")
                    .multiple(true)
                    .help("Increase logging verbosity"),
            )
            .arg(
                Arg::with_name("timestamp")
                    .long("timestamp")
                    .help("Prepend log lines with a timestamp"),
            )
            .arg(
                Arg::with_name("db_dir")
                    .long("db-dir")
                    .help("Directory to store index database (default: ./db/)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("daemon_dir")
                    .long("daemon-dir")
                    .help("Data directory of Bitcoind (default: ~/.bitcoin/)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("cookie")
                    .long("cookie")
                    .help("JSONRPC authentication cookie ('USER:PASSWORD', default: read from ~/.bitcoin/.cookie)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("network")
                    .long("network")
                    .help(&network_help)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("electrum_rpc_addr")
                    .long("electrum-rpc-addr")
                    .help("Electrum server JSONRPC 'addr:port' to listen on (default: '127.0.0.1:50001' for mainnet, '127.0.0.1:60001' for testnet and '127.0.0.1:60401' for regtest)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("http_addr")
                    .long("http-addr")
                    .help("HTTP server 'addr:port' to listen on (default: '127.0.0.1:3000' for mainnet, '127.0.0.1:3001' for testnet and '127.0.0.1:3002' for regtest)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("daemon_rpc_addr")
                    .long("daemon-rpc-addr")
                    .help("Bitcoin daemon JSONRPC 'addr:port' to connect (default: 127.0.0.1:8332 for mainnet, 127.0.0.1:18332 for testnet and 127.0.0.1:18443 for regtest)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("monitoring_addr")
                    .long("monitoring-addr")
                    .help("Prometheus monitoring 'addr:port' to listen on (default: 127.0.0.1:4224 for mainnet, 127.0.0.1:14224 for testnet and 127.0.0.1:24224 for regtest)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("jsonrpc_import")
                    .long("jsonrpc-import")
                    .help("Use JSONRPC instead of directly importing blk*.dat files. Useful for remote full node or low memory system"),
            )
            .arg(
                Arg::with_name("index_batch_size")
                    .long("index-batch-size")
                    .help("Number of blocks to get in one JSONRPC request from bitcoind")
                    .default_value("100"),
            )
            .arg(
                Arg::with_name("bulk_index_threads")
                    .long("bulk-index-threads")
                    .help("Number of threads used for bulk indexing (default: use the # of CPUs)")
                    .default_value("0")
            )
            .arg(
                Arg::with_name("tx_cache_size")
                    .long("tx-cache-size")
                    .help("Number of transactions to keep in for query LRU cache")
                    .default_value("10000")  // should be enough for a small wallet.
            )
            .arg(
                Arg::with_name("disable_prevout")
                    .long("disable-prevout")
                    .help("Don't attach previous output details to inputs")
            )
            .arg(
                Arg::with_name("cors")
                    .long("cors")
                    .help("Origins allowed to make cross-site requests")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("precache_scripts")
                    .long("precache-scripts")
                    .help("Path to file with list of scripts to pre-cache")
                    .takes_value(true)
            );

        #[cfg(any(feature = "ocean", feature = "liquid"))]
        let args = args
            .arg(
                Arg::with_name("parent_network")
                    .long("parent-network")
                    .help("Select parent network type (mainnet, testnet, regtest)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("asset_db_path")
                    .long("asset-db-path")
                    .help("Directory for ocean/elements asset db")
                    .takes_value(true),
            );

        let m = args.get_matches();

        let network_name = m.value_of("network").unwrap_or("mainnet");
        let network_type = Network::from(network_name);
        let db_dir = Path::new(m.value_of("db_dir").unwrap_or("./db"));
        let db_path = db_dir.join(network_name);

        #[cfg(any(feature = "ocean", feature = "liquid"))]
        let parent_network = Network::from(m.value_of("parent_network").unwrap_or("mainnet"));
        #[cfg(any(feature = "ocean", feature = "liquid"))]
        let parent_genesis_hash = parent_network.genesis_hash().to_hex();
        #[cfg(any(feature = "ocean", feature = "liquid"))]
        let asset_db_path = m.value_of("asset_db_path").map(PathBuf::from);

        let default_daemon_port = match network_type {
            Network::Bitcoin => 8332,
            Network::Testnet => 18332,
            Network::Regtest => 18443,

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => 7041,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => 7041,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => 7041,
        };
        let default_electrum_port = match network_type {
            Network::Bitcoin => 50001,
            Network::Testnet => 60001,
            Network::Regtest => 60401,

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => 51000,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => 51000,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => 51401,
        };
        let default_http_port = match network_type {
            Network::Bitcoin => 3000,
            Network::Testnet => 3001,
            Network::Regtest => 3002,

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => 3000,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => 3000,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => 3002,
        };
        let default_monitoring_port = match network_type {
            Network::Bitcoin => 4224,
            Network::Testnet => 14224,
            Network::Regtest => 24224,

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => 34224,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => 34224,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => 44224,
        };

        let daemon_rpc_addrs: Vec<_> = m
            .value_of("daemon_rpc_addr")
            .unwrap_or(&format!("127.0.0.1:{}", default_daemon_port))
            .to_socket_addrs()
            .expect("invalid Bitcoind RPC address")
            .collect();
        let daemon_rpc_addr : SocketAddr = daemon_rpc_addrs[0];

        let electrum_rpc_addrs: Vec<_> = m
            .value_of("electrum_rpc_addr")
            .unwrap_or(&format!("127.0.0.1:{}", default_electrum_port))
            .to_socket_addrs()
            .expect("invalid Electrum RPC address")
            .collect();
        let electrum_rpc_addr : SocketAddr = electrum_rpc_addrs[0];

        let http_addrs: Vec<_> = m
            .value_of("http_addr")
            .unwrap_or(&format!("127.0.0.1:{}", default_http_port))
            .to_socket_addrs()
            .expect("invalid HTTP server address")
            .collect();
        let http_addr : SocketAddr = http_addrs[0];

        let monitoring_addrs: Vec<_> = m
            .value_of("monitoring_addr")
            .unwrap_or(&format!("127.0.0.1:{}", default_monitoring_port))
            .to_socket_addrs()
            .expect("invalid Prometheus monitoring address")
            .collect();
        let monitoring_addr : SocketAddr = monitoring_addrs[0];

        let mut daemon_dir = m
            .value_of("daemon_dir")
            .map(|p| PathBuf::from(p))
            .unwrap_or_else(|| {
                let mut default_dir = home_dir().expect("no homedir");
                default_dir.push(".bitcoin");
                default_dir
            });
        match network_type {
            Network::Bitcoin => (),
            Network::Testnet => daemon_dir.push("testnet3"),
            Network::Regtest => daemon_dir.push("regtest"),

            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Ocean => daemon_dir.push("ocean_main"),
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::Gold => daemon_dir.push("gold_main"),
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            Network::OceanRegtest => daemon_dir.push("oceanregtest"),
        }
        let cookie = m.value_of("cookie").map(|s| s.to_owned());

        let mut log = stderrlog::new();
        log.verbosity(m.occurrences_of("verbosity") as usize);
        log.timestamp(if m.is_present("timestamp") {
            stderrlog::Timestamp::Millisecond
        } else {
            stderrlog::Timestamp::Off
        });
        log.init().expect("logging initialization failed");
        let mut bulk_index_threads = value_t_or_exit!(m, "bulk_index_threads", usize);
        if bulk_index_threads == 0 {
            bulk_index_threads = num_cpus::get();
        }
        let config = Config {
            log,
            network_type,
            db_path,
            daemon_dir,
            daemon_rpc_addr,
            cookie,
            electrum_rpc_addr,
            http_addr,
            monitoring_addr,
            jsonrpc_import: m.is_present("jsonrpc_import"),
            index_batch_size: value_t_or_exit!(m, "index_batch_size", usize),
            bulk_index_threads,
            tx_cache_size: value_t_or_exit!(m, "tx_cache_size", usize),
            prevout_enabled: !m.is_present("disable_prevout"),
            cors: m.value_of("cors").map(|s| s.to_string()),
            precache_scripts: m.value_of("precache_scripts").map(|s| s.to_string()),
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            parent_network,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            parent_genesis_hash,
            #[cfg(any(feature = "ocean", feature = "liquid"))]
            asset_db_path,
        };
        eprintln!("{:?}", config);
        config
    }

    pub fn cookie_getter(&self) -> Arc<CookieGetter> {
        if let Some(ref value) = self.cookie {
            Arc::new(StaticCookie {
                value: value.as_bytes().to_vec(),
            })
        } else {
            Arc::new(CookieFile {
                daemon_dir: self.daemon_dir.clone(),
            })
        }
    }
}

struct StaticCookie {
    value: Vec<u8>,
}

impl CookieGetter for StaticCookie {
    fn get(&self) -> Result<Vec<u8>> {
        Ok(self.value.clone())
    }
}

struct CookieFile {
    daemon_dir: PathBuf,
}

impl CookieGetter for CookieFile {
    fn get(&self) -> Result<Vec<u8>> {
        let path = self.daemon_dir.join(".cookie");
        let contents = fs::read(&path).chain_err(|| {
            ErrorKind::Connection(format!("failed to read cookie from {:?}", path))
        })?;
        Ok(contents)
    }
}

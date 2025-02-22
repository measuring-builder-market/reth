use alloy_genesis::Genesis;
use reth_chainspec::{ChainSpec, DEV, HOLESKY, MAINNET, SEPOLIA};
use reth_cli::chainspec::ChainSpecParser;
use std::{fs, path::PathBuf, sync::Arc};

/// Clap value parser for [`ChainSpec`]s.
///
/// The value parser matches either a known chain, the path
/// to a json file, or a json formatted string in-memory. The json needs to be a Genesis struct.
fn chain_value_parser(s: &str) -> eyre::Result<Arc<ChainSpec>, eyre::Error> {
    Ok(match s {
        "mainnet" => MAINNET.clone(),
        "sepolia" => SEPOLIA.clone(),
        "holesky" => HOLESKY.clone(),
        "dev" => DEV.clone(),
        _ => {
            // try to read json from path first
            let raw = match fs::read_to_string(PathBuf::from(shellexpand::full(s)?.into_owned())) {
                Ok(raw) => raw,
                Err(io_err) => {
                    // valid json may start with "\n", but must contain "{"
                    if s.contains('{') {
                        s.to_string()
                    } else {
                        return Err(io_err.into()) // assume invalid path
                    }
                }
            };

            // both serialized Genesis and ChainSpec structs supported
            let genesis: Genesis = serde_json::from_str(&raw)?;

            Arc::new(genesis.into())
        }
    })
}

/// Ethereum chain specification parser.
#[derive(Debug, Clone, Default)]
pub struct EthChainSpecParser;

impl ChainSpecParser for EthChainSpecParser {
    type ChainSpec = ChainSpec;

    const SUPPORTED_CHAINS: &'static [&'static str] = &["mainnet", "sepolia", "holesky", "dev"];

    fn parse(s: &str) -> eyre::Result<Arc<ChainSpec>> {
        chain_value_parser(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_chain_spec() {
        for &chain in EthChainSpecParser::SUPPORTED_CHAINS {
            assert!(<EthChainSpecParser as ChainSpecParser>::parse(chain).is_ok());
        }
    }
}

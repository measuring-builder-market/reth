# Reth 

## Installation

Please follow the [official documentation](https://reth.rs/installation/source.html) to build the modified Reth from source code.

## Block Simulation

We modified Reth to introduce a new JSON-RPC API method, `eth_simulateBlock`, which can be used to simulate a new block against a specific state. Below is an example using Python to invoke this method.

```python
import json
import requests

from web3 import Web3

# send rpc request to API eth_simulateBlock
def rpc_simulate_block(url, raw_transactions, block_number, base_fee, coinbase, addresses, timestamp):
    payload = json.dumps({
        "method": "eth_simulateBlock",
        "params": [
            {
              "txs":  raw_transactions,
              "blockNumber": block_number,
              "coinbase": Web3.to_checksum_address(coinbase),
              "baseFee": base_fee,
              "stateBlockNumber": hex(block_number-1),
              "builderAddresses": addresses,
              "timestamp": timestamp
          }],
        "id": 1,
        "jsonrpc": "2.0"
    })
    
    headers = {
        'Content-Type': 'application/json'
    }

    response = requests.post(url, headers=headers, data=payload, timeout=60)

    return response.json()
```

### Parameters for `eth_simulateBlock`

- **`txs`**:  A list of hex-encoded signed transactions that will be included in the simulated block.
- **`blockNumber`**: The block number for which this block is valid on.
- **`coinbase`**:  A checksummed Ethereum address of the block fee recipient for the simulated block.
- **`baseFee`**: The base fee of the block to use for this simulation.
- **`stateBlockNumber`**: Either a hex encoded number or a block tag for which state to base this simulation on.
- **`builderAddresses`**: A list of addresses of the builder.
- **`timestamp`**: The timestamp to use for this block simulation, in seconds since the unix epoch.

### Notes

To enable block simulation against any block state, you should run Reth as an archive node.
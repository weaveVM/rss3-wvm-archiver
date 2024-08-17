<p align="center">
  <a href="https://wvm.dev">
    <img src="./media/banner.png">
  </a>
</p>

## About
WeaveVM Archiver is an ETL archive pipeline for EVM networks. It's the simplest way to interface with WeaveVM's permanent data feature without smart contract redeployments.

## About RSS3 Network
The [RSS3 Network](https://rss3.io) is a decentralized network designed to promote the free flow of information on the Open Web .

## rss3-wvm-archiver node configuration

- node endpoint: https://rss3-wvm-archiver.shuttleapp.rs
- wvm-archiver node version: [v0.1.2](https://github.com/weaveVM/wvm-archiver/releases/tag/v0.1.2)

### Node configuration

```json
{
    "name": "RSS3 VSL Mainnet",
    "network_chain_id": 12553,
    "wvm_chain_id": 9496,
    "network_rpc": "https://rpc.rss3.io",
    "wvm_rpc": "https://testnet-rpc.wvm.dev",
    "block_time": 2,
    "start_block": 6888111,
    "archiver_address": "0xA6dC883ea2A6acb576A933B4d38D13d6069d9fBE",
    "archive_pool_address": "0x0000000000000000000000000000000000000000"
}
```

# Generic documentation content from [wvm-archiver](https://github.com/weaveVM/wvm-archiver) repo

### WeaveVM Archiver Usage

WeaveVM Archiver is the ideal choice if you want to:

- Interface with WeaveVM's permanent data settlement and high-throughput DA
- Maintain your current data settlement or DA architecture
- Have an interface with WeaveVM without rollup smart contract redeployments
- Avoid codebase refactoring

## Build & Run

```bash
git clone https://github.com/weaveVM/rss3-wvm-archiver.git

cd wvm-archiver

cargo shuttle run
```

### Prerequisites & Dependencies

While a WeaveVM Archiver node can run without web2 component dependencies, this node implementation uses [planetscale](https://planetscale.com) for cloud indexing (indexing target network block ID to WVM archive TXID) and [shuttle.rs](https://shuttle.rs) for backend hosting. Check [.env.example](./env.example) to set up your environment variables.

```js
archiver_pk="" // WeaveVM archiver PK
network="./networks/your_network.json"

DATABASE_HOST="" // planetscale
DATABASE_USERNAME="" // planetscale
DATABASE_PASSWORD="" // planetscale
```

### Add Your Network

To start archiving your network block data on WeaveVM:

1. Add your network config file to the [networks](./networks/) directory.
2. Name your config file using snake_case syntax (e.g., `your_network_name.json`).
3. Modify properties that don't have a `wvm_` prefix in the config JSON file.
4. Fund your `archiver_address` with a sufficient amount of tWVM (1 MB costs ~ 5 cents). Check out WVM Faucet to claim $tWVM.
5. Choose a unique `archive_pool_address` that's different from your `archiver_address`.
6. Set up your PlanetScale DB according to `db_schema.sql`.

### RPC Proxy and Caching

You can use [eRPC](https://github.com/erpc/erpc) to cache, load-balance and failover between as many RPC endpoints and use eRPC's proxy URL in each network's config for WeaveVM. This will increase performance and resiliency and reduce RPC usage cost while fetching network's block data via WeaveVM.

```bash
# modify erpc.yaml
cp erpc.yaml.dist erpc.yaml
code erpc.yaml

# run docker-compose
docker-compose up -d
```

Finally, you can set eRPC's proxy URL in each relative network config.
```optimism.json
{
    "name": "Optimism",
    "network_chain_id": 10,
    "network_rpc": "http://erpc:4000/main/evm/10",
    ...
}
```


## How it works

The WeaveVM Archiver node operates as follows:

1. It starts downloading the target EVM network block data from the RPC you provide in the network config file.
2. The node begins pulling blocks from the `start_block` defined in the network's config file.
3. The block data is then serialized in [borsh](https://borsh.io) format and compressed using Brotli.
4. The serialized-compressed data is pushed to WeaveVM as calldata transaction from the `archiver_address` to the `archive_pool_address`.
5. Simultaneously, the resulting TXID from pushing data to WeaveVM and the archived EVM block ID are indexed in the cloud for faster data retrieval.

## Server Methods

As mentioned, PlanetScale is used for cloud indexing, which allows a WeaveVM Archiver node to expose its WeaveVM data as a RESTful API.

### WeaveVM Archiver node instance info

```bash
curl -X GET https://your_app.shuttleapp.rs/info
```
**returns:**

```rs
pub struct InfoServerResponse {
    first_block: Option<u64>,
    last_block: Option<u64>,
    total_archived_blocks: u64,
    archiver_balance: U256,
    archiver_address: String,
    network_name: String,
    network_chain_id: u32,
    network_rpc: String,
}
```

### Retrieve the WVM archive TXID for a given EVM block ID

```bash
curl -X GET https://your_app.shuttleapp.rs/block/$BLOCK_ID
```

### Decode the WVM archived block data for a given EVM block ID (return original block data in JSON format)

```bash
curl -X GET https://your_app.shuttleapp.rs/block/raw/$BLOCK_ID
```

## License
This project is licensed under the [MIT License](./LICENSE)
# Changelog

## [unreleased]

### Protocol Changes
**No Changes**

### Non-protocol Changes
* Moved Tier1 configuration from experimental to top level config. No action is necessary as the default values are the recommended ones. ([#13575](https://github.com/near/nearcore/pull/13575))

## [2.7.0]

### Protocol Changes

* A new shard layout for production networks ([#13324](https://github.com/near/nearcore/pull/13324)). Use split boundary from [#13609](https://github.com/near/nearcore/pull/13609).
* When the protocol update version voting takes place, validators that did not upgrade to the latest version will be scheduled for removal (aka kickout) in the epoch the new version takes effect. This helps avoid missed blocks in the first epoch of the new version, as un-upgraded validators would produce invalid blocks. Technically this is a protocol change as it impacts the validator set, however it will take effect during the next version upgrade therefore does not require its own protocol version. ([#13375](https://github.com/near/nearcore/issues/13375))
* Implement [NEP-536](https://github.com/near/NEPs/pull/536): Reduce the number of refund receipts by adding removing pessimistic gas pricing. Also introduce a gas refund penalty but set it to 0 to avoid potential negative impact. ([#13397](https://github.com/near/nearcore/issues/13397))
* Implement P2P sync for state sync headers. ([#13377](https://github.com/near/nearcore/pull/13377))
* Enable saturating float-to-int conversions in runtime. ([#13414](https://github.com/near/nearcore/pull/13414))

### Non-protocol Changes
* Add RPC query for viewing global contract code. ([#13547](https://github.com/near/nearcore/pull/13547))
* Add promise batch host functions for global contracts. ([#13565](https://github.com/near/nearcore/pull/13565))
* Stabilize `EXPERIMENTAL_changes` RPC method and rename it to `changes`. ([#13722](https://github.com/near/nearcore/pull/13722))

## [2.6.0]

### Protocol Changes

* Implemented support for global contracts: [NEP-591](https://github.com/near/NEPs/pull/591)
* Implemented Optimistic Block to remove doubled chunk execution latency from block & chunk production flow [#10584](https://github.com/near/nearcore/issues/10584)
* Changed receipt id computation to enable chunk execution based on Optimistic Block. More specifically, primitive [`create_hash_upgradable`](https://github.com/near/nearcore/blob/700735b/core/primitives/src/utils.rs#L292-L309) is changed to use `block_height` instead of `extra_hash`.

### Non-protocol Changes
**No Changes**

## [2.5.0]

### Protocol Changes
* Add cross-shard bandwidth scheduler which manages transferring receipts between shards,
  enabling higher throughput of cross-shard receipts and better horizontal scalability.
  NEP-584 (https://github.com/near/NEPs/pull/584)
* Resharding V3 - a new implementation for resharding and two new shard layouts
  for the production networks.
  NEP-568 (https://github.com/near/NEPs/pull/568)

### Non-protocol Changes
* Parallelize transaction validation (including signature checks) before `verify_and_charge_transaction`,
  significantly improving throughput for transaction processing on the nodes. [#12654](https://github.com/near/nearcore/pull/12654)
* Current Epoch State Sync - Moves the sync point from the previous epoch to the
  current epoch. [#12102](https://github.com/near/nearcore/pull/12102)

## 2.4.0

### Protocol Changes

* Fixing invalid cost used for `wasm_yield_resume_byte`. [#12192](https://github.com/near/nearcore/pull/12192)
* Relaxing Congestion Control to allow accepting and buffering more transactions. [#12241](https://github.com/near/nearcore/pull/12241) [#12430](https://github.com/near/nearcore/pull/12430)
* Exclude contract code out of state witness and distribute it separately. [#11099](https://github.com/near/nearcore/issues/11099)

### Non-protocol Changes
* **Epoch Sync V4**: A capability to bootstrap a node from another active node. [#73](https://github.com/near/near-one-project-tracking/issues/73)
* **Decentralized state sync**: Before, nodes that needed to download state
(either because they're several epochs behind the chain or because they're going to start producing chunks for a shard they don't currently track)
would download them from a centralized GCS bucket. Now, nodes will attempt to download pieces of the state from peers in the network,
and only fallback to downloading from GCS if that fails. Please note that in order to participate in providing state parts to peers,
your node may generate snapshots of the state. These snapshots should not take too much space,
since they're hard links to database files that get cleaned up on every epoch. [#12004](https://github.com/near/nearcore/issues/12004)

## 2.3.0

### Protocol Changes
* Sets `chunk_validator_only_kickout_threshold` to 70. Uses this kickout threshold as a cutoff threshold for contribution of endorsement ratio in rewards calculation: if endorsement ratio is above 70%, the contribution of endorsement ratio in average uptime calculation is 100%, otherwise it is 0%. Endorsements received are now included in `BlockHeader` to improve kickout and reward calculation for chunk validators. 

### Non-protocol Changes
* Added [documentation](./docs/misc/archival_data_recovery.md) and a [reference](./scripts/recover_missing_archival_data.sh) script to recover the data lost in archival nodes at the beginning of 2024.
* **Archival nodes only:** Stop saving partial chunks to `PartialChunks` column in the Cold DB. Instead, archival nodes will reconstruct partial chunks from the `Chunks` column.
* Enabled state snapshots on every epoch to allow the nodes to take part in decentralized state sync in future releases.

## 2.2.1

This release patches a bug found in the 2.2.0 release

# Non-protocol changes
There was a bug in the integration between ethereum implicit accounts and the compiled contract cache which sometimes caused the nodes to get stuck. This would most often happen during state sync, but could also happen by itself. Please update your nodes to avoid getting stuck.

A node that hits this bug will print an error about an `InvalidStateRoot` in the logs and then it'll be unable to sync.
It's possible to recover a stalled node by clearing the compiled contract cache and rolling back one block:
1. Stop the neard process
2. Download the new version of neard
3. Clear the compiled contract cache: rm -rf ~/.near/data/contracts
4. Undo the last block: ./neard undo-block
5. Start neard

After that the node should be able to recover and sync with the rest of the network.

## 2.2.0

### Protocol Changes
* The minimum validator stake has been set to a lower value. The small-stake validators that were kicked out during the shift to stateless validation will be able to rejoin the network.
* Better algorithm for validator kickouts
* (Testnet only) update the eth-implicit accounts contract on testnet to match the one on mainnet.

### Non-protocol Changes
* Fix spammy messages about calculating gas for PromiseYield receipts.
* Don't crash when the CPU doesn't have SHA-NI instructions. It's still a hardware requirement, there is no guarantee that nodes without this instruction will be able to keep up with the network, but `neard` will now be able to run (slowly) on CPUs without this instruction.

## 2.1.0

### Protocol Changes
* Eth-Implicit Accounts [NEP-0518](https://github.com/near/NEPs/pull/518)
* Host Functions for BLS12-381 Curve Operations [NEP-0488](https://github.com/near/NEPs/pull/488)

### Non-protocol Changes

* Enforce rate limits to received network messages [#11617](https://github.com/near/nearcore/issues/11617). Rate limits are configured by default, but they can be overridden through the experimental configuration option `received_messages_rate_limits`.

* Increase sync blocks requested and make it configurable through a parameter in the client config [#11820](https://github.com/near/nearcore/pull/11820). Increase default max sync block requests to 10 from 5.

## 2.0.0

### Protocol Changes
* Congestion Control [NEP-0539](https://github.com/near/NEPs/pull/539)
* Stateless Validation [NEP-0509](https://github.com/near/NEPs/pull/509)

### Non-protocol Changes
**No Changes**

## 1.40.0

### Protocol Changes

**No Changes**

### Non-protocol Changes

## 1.39.0

### Protocol Changes

* Use more precise gas costs for function calls [#10943](https://github.com/near/nearcore/pull/10943) that should lead to more efficient chunk utilization. 

### Non-protocol Changes

* Limit overcharging by decoupling minimum_new_receipt_gas from the function call and setting it to a constant value. [#10941](https://github.com/near/nearcore/pull/10941)
* These PRs introduce a change in the default behaviour of `broadcast_tx_commit`,`send_tx`, `tx, EXPERIMENTAL_tx_status` RPC methods. The default behaviour no longer waits for refund receipts.
  If you do need to wait for refund receipts, you need ask about it explicitly by using TxExecutionStatus::Final option ("wait_until": "FINAL" in the json request). More information in the
  [#10792](https://github.com/near/nearcore/pull/10792) [#10948](https://github.com/near/nearcore/pull/10948)
* Adds improvements to the `sweat` contract prefetcher logic. Add new prefetcher logic for `kaiching` contract. [#10899](https://github.com/near/nearcore/pull/10899)
* Improves prefetcher logic to speedup chunk finalization by prefetching keys related to refund receipts and actions such as: Delegate, AddKey, DeleteKey. [#10936](https://github.com/near/nearcore/pull/10936)
* Add more metrics for receipt processing. [#10944](https://github.com/near/nearcore/pull/10944)

## 1.37.0

### Protocol Changes

* Resharding v2 - new implementation for resharding and a new shard layout for production networks. [#10303](https://github.com/near/nearcore/pull/10303), [NEP-0508](https://github.com/near/NEPs/pull/508)
* Restrict the creation of non-implicit top-level account that are longer than 32 bytes. Only the registrar account can create them. [#9589](https://github.com/near/nearcore/pull/9589)
* Adjust the number of block producers and chunk producers on testnet to facilitate testing of chunk-only producers [#9563](https://github.com/near/nearcore/pull/9563)


### Non-protocol Changes

* Add prometheus metrics for the internal state of the doomslug. [#9458](https://github.com/near/nearcore/pull/9458)
* Fix `EXPERIMENTAL_protocol_config` to apply overrides from `EpochConfig`. [#9692](https://github.com/near/nearcore/pull/9692)
* Add config option `tx_routing_height_horizon` to configure how many chunk producers are notified about the tx. [#10251](https://github.com/near/nearcore/pull/10251)

## 1.36.0

### Protocol Changes

* The support for fixed shards in shard layout was removed. [#9219](https://github.com/near/nearcore/pull/9219)

### Non-protocol Changes

* New option `transaction_pool_size_limit` in `config.json` allows to limit the size of the node's transaction pool.
  By default the limit is set to 100 MB. [#3284](https://github.com/near/nearcore/issues/3284)
* Database snapshots at the end of an epoch. This lets a node obtain state parts using flat storage. [#9090](https://github.com/near/nearcore/pull/9090)
* Number of transactions included in a chunk will be lowered if there is a congestion of more than 20000 delayed receipts in a shard. [#9222](https://github.com/near/nearcore/pull/9222)
* Our more efficient and scalable V2 routing protocol is implemented. It shadows the V1 protocol for now while we verify its performance. [#9187](https://github.com/near/nearcore/pull/9187)
* The default config now enables TIER1 outbound connections by default. [#9349](https://github.com/near/nearcore/pull/9349)
* State Sync from GCS is available for experimental use. [#9398](https://github.com/near/nearcore/pull/9398)

## 1.35.0

### Protocol Changes

* Upgrade the contract preparation code to use [finite-wasm](https://github.com/near/finite-wasm), which guarantees deterministic limits on execution time and space of compiled contracts

### Non-protocol Changes

* Dump state by multiple nodes, each node will refer to s3 for which parts need to be dumped. [#9049](https://github.com/near/nearcore/pull/9049)
* Small values in the flat storage trie are inlined for faster accesses [#9029](https://github.com/near/nearcore/pull/9029)
* A current protocol version metric is added to the prometheus metrics under near_current_protocol_version [#9030](https://github.com/near/nearcore/pull/9030)
* The transaction pool size is tracked, and if the `transaction_pool_size_limit` config option is set, we now avoid storing more than the specified size of transactions in each shard's transaction pool [#8970](https://github.com/near/nearcore/pull/8970) and [#9036](https://github.com/near/nearcore/pull/9036)

## 1.34.0

### Protocol Changes

* Flat Storage for reads, reducing number of DB accesses for state read from `2 * key.len()` in the worst case to 2. [#8761](https://github.com/near/nearcore/pull/8761), [NEP-399](https://github.com/near/NEPs/pull/399)
* Contract preparation and gas charging for wasm execution also switched to using our own code, as per the finite-wasm specification. Contract execution gas costs will change slightly for expected use cases. This opens up opportunities for further changing the execution gas costs (eg. with different costs per opcode) to lower contract execution cost long-term. [#8912](https://github.com/near/nearcore/pull/8912)
* Compute Costs are implemented and stabilized. Compute usage of the chunk is now limited according to the compute costs. [#8915](https://github.com/near/nearcore/pull/8915), [NEP-455](https://github.com/near/NEPs/blob/master/neps/nep-0455.md).
* Write related storage compute costs are increased which means they fill a chunk sooner but gas costs are unaffected. [#8924](https://github.com/near/nearcore/pull/8924)

### Non-protocol Changes

* undo-block tool to reset the chain head from current head to its prev block. Use the tool by running: `./target/release/neard undo-block`. [#8681](https://github.com/near/nearcore/pull/8681)
* Add prometheus metrics for expected number of blocks/chunks at the end of the epoch. [#8759](https://github.com/near/nearcore/pull/8759)
* Node can sync State from S3. [#8789](https://github.com/near/nearcore/pull/8789)
* Node can sync State from local filesystem. [#8913](https://github.com/near/nearcore/pull/8913)
* Add per shard granularity for chunks in validator info metric. [#8934](https://github.com/near/nearcore/pull/8934)

## 1.33.0

### Protocol Changes

### Non-protocol Changes
* State-viewer tool to dump and apply state changes from/to a range of blocks. [#8628](https://github.com/near/nearcore/pull/8628)
* Experimental option to dump state of every epoch to external storage. [#8661](https://github.com/near/nearcore/pull/8661)
* Add prometheus metrics for tracked shards, block height within epoch, if is block/chunk producer. [#8728](https://github.com/near/nearcore/pull/8728)
* State sync is disabled by default [#8730](https://github.com/near/nearcore/pull/8730)
* Node can restart if State Sync gets interrupted. [#8732](https://github.com/near/nearcore/pull/8732)
* Merged two `neard view-state` commands: `apply-state-parts` and `dump-state-parts` into a single `state-parts` command. [#8739](https://github.com/near/nearcore/pull/8739)
* Add config.network.experimental.network_config_overrides to the JSON config. [#8871](https://github.com/near/nearcore/pull/8871)

## 1.32.2

### Fixes
* Fix: rosetta zero balance accounts [#8833](https://github.com/near/nearcore/pull/8833)

## 1.32.1

### Fixes
* Fix vulnerabilities in block outcome root validation and total supply validation [#8790](https://github.com/near/nearcore/pull/8790)

## 1.32.0

### Protocol Changes
* Stabilize `ed25519_verify` feature: introducing a host function to verify
ed25519 signatures efficiently.
[#8098](https://github.com/near/nearcore/pull/8098)
[NEP-364](https://github.com/near/NEPs/pull/364)
* Added STUN-based self-discovery to make configuration of TIER1 network easier in the simplest validator setups.
  [#8472](https://github.com/near/nearcore/pull/8472)
* Stabilize zero balance account feature: allows account to not hold balance under certain conditions
and enables a more smooth onboarding experience where users don't have to first acquire NEAR tokens
to pay for the storage of their accounts.
[#8378](https://github.com/near/nearcore/pull/8378)
[NEP-448](https://github.com/near/NEPs/pull/448)
* Stabilize meta transactions on the protocol level.
[NEP-366](https://github.com/near/NEPs/blob/master/neps/nep-0366.md),
[Tracking issue #8075](https://github.com/near/nearcore/issues/8075),
[Stabilization #8601](https://github.com/near/nearcore/pull/8601)

### Non-protocol Changes
* Config validation can be done by following command:
  `./target/debug/neard --home {path_to_config_files} validate-config`.
  This will show error if there are file issues or semantic issues in `config.json`, `genesis.json`, `records.json`, `node_key.json` and `validator_key.json`.
  [#8485](https://github.com/near/nearcore/pull/8485)
* Comments are allowed in configs. This includes
  `config.json`, `genesis.json`, `node_key.json` and `validator_key.json`. You can use `//`, `#` and `/*...*/` for comments.
  [#8423](https://github.com/near/nearcore/pull/8423)
* `/debug` page now has client_config linked.
  You can also check your client_config directly at /debug/client_config
  [#8400](https://github.com/near/nearcore/pull/8400)
* Added cold store loop - a background thread that copies data from hot to cold storage and a new json rpc endpoint - split_storage_info - that
  exposes debug info about the split storage.
  [#8432](https://github.com/near/nearcore/pull/8432)
* `ClientConfig` can be updated while the node is running.
  `dyn_config.json` is no longer needed as its contents were merged into `config.json`.
  [#8240](https://github.com/near/nearcore/pull/8240)
* TIER2 network stabilization. Long-lasting active connections are persisted to DB and are re-established automatically if either node restarts. A new neard flag `--connect-to-reliable-peers-on-startup` is provided to toggle this behavior; it defaults to true. The PeerStore is no longer persisted to DB and is now kept in-memory. [#8579](https://github.com/near/nearcore/issues/8579), [#8580](https://github.com/near/nearcore/issues/8580).

## 1.31.0

### Non-protocol Changes

* Enable TIER1 network. Participants of the BFT consensus (block & chunk producers) now can establish direct TIER1 connections
  between each other, which will optimize the communication latency and minimize the number of dropped chunks.
  To configure this feature, see [advanced\_configuration/networking](./docs/advanced_configuration/networking.md).
  [#8141](https://github.com/near/nearcore/pull/8141)
  [#8085](https://github.com/near/nearcore/pull/8085)
  [#7759](https://github.com/near/nearcore/pull/7759)
* [Network] Started creating connections with larger nonces, that are periodically
  refreshed Start creating connections (edges) with large nonces
  [#7966](https://github.com/near/nearcore/pull/7966)
* `/status` response has now two more fields: `node_public_key` and
  `validator_public_key`.  The `node_key` field is now deprecated and should not
  be used since it confusingly holds validator key.
  [#7828](https://github.com/near/nearcore/pull/7828)
* Added `near_node_protocol_upgrade_voting_start` Prometheus metric whose value
  is timestamp when voting for the next protocol version starts.
  [#7877](https://github.com/near/nearcore/pull/7877)
* neard cmd can now verify proofs from JSON files.
  [#7840](https://github.com/near/nearcore/pull/7840)
* In storage configuration, the value `trie_cache_capacities` now is no longer
  a hard limit but instead sets a memory consumption limit. For large trie nodes,
  the limits are close to equivalent. For small values, there can now fit more
  in the cache than previously.
  [#7749](https://github.com/near/nearcore/pull/7749)
* New options `store.trie_cache` and `store.view_trie_cache` in `config.json`
  to set limits on the trie cache. Deprecates the never announced
  `store.trie_cache_capacities` option which was mentioned in previous change.
  [#7578](https://github.com/near/nearcore/pull/7578)
* New option `store.background_migration_threads` in `config.json`. Defines
  number of threads to execute background migrations of storage. Currently used
  for flat storage migration. Set to 8 by default, can be reduced if it slows down
  block processing too much or increased if you want to speed up migration.
  [#8088](https://github.com/near/nearcore/pull/8088),
* Tracing of work across actix workers within a process:
  [#7866](https://github.com/near/nearcore/pull/7866),
  [#7819](https://github.com/near/nearcore/pull/7819),
  [#7773](https://github.com/near/nearcore/pull/7773).
* Scope of collected tracing information can be configured at run-time:
  [#7701](https://github.com/near/nearcore/pull/7701).
* Attach node's `chain_id`, `node_id`, and `account_id` values to tracing
  information: [#7711](https://github.com/near/nearcore/pull/7711).
* Change exporter of tracing information from `opentelemetry-jaeger` to
  `opentelemetry-otlp`: [#7563](https://github.com/near/nearcore/pull/7563).
* Tracing of requests across processes:
  [#8004](https://github.com/near/nearcore/pull/8004).
* Gas profiles as displayed in the `EXPERIMENTAL_tx_status` are now more
  detailed and give the gas cost per parameter.

## 1.30.0

### Protocol Changes

* Stabilize `account_id_in_function_call_permission` feature: enforcing validity
  of account ids in function call permission.
  [#7569](https://github.com/near/nearcore/pull/7569)

### Non-protocol Changes

* `use_db_migration_snapshot` and `db_migration_snapshot_path` options are now
  deprecated.  If they are set in `config.json` the node will fail if migration
  needs to be performed.  Use `store.migration_snapshot` instead to configure
  the behaviour [#7486](https://github.com/near/nearcore/pull/7486)
* Added `near_peer_message_sent_by_type_bytes` and
  `near_peer_message_sent_by_type_total` Prometheus metrics measuring
  size and number of messages sent to peers.
  [#7523](https://github.com/near/nearcore/pull/7523)
* `near_peer_message_received_total` Prometheus metric is now deprecated.
  Instead of it aggregate `near_peer_message_received_by_type_total` metric.
  For example, to get total rate of received messages use
  `sum(rate(near_peer_message_received_by_type_total{...}[5m]))`.
  [#7548](https://github.com/near/nearcore/pull/7548)
* Few changes to `view_state` JSON RPC query:
  - The request has now an optional `include_proof` argument.  When set to
    `true`, response’s `proof` will be populated.
  - The `proof` within each value in `values` list of a `view_state` response is
    now deprecated and will be removed in the future.  Client code should ignore
    the field.
  - The `proof` field directly within `view_state` response is currently always
    sent even if proof has not been requested.  In the future the field will be
    skipped in those cases.  Clients should accept responses with this field
    missing (unless they set `include_proof`).
    [#7603](https://github.com/near/nearcore/pull/7603)
* Backtraces on panics are enabled by default, so you no longer need to set
  `RUST_BACKTRACE=1` environmental variable. To disable backtraces, set
  `RUST_BACKTRACE=0`. [#7562](https://github.com/near/nearcore/pull/7562)
* Enable receipt prefetching by default. This feature makes receipt processing
  faster by parallelizing IO requests, which has been introduced in
  [#7590](https://github.com/near/nearcore/pull/7590) and enabled by default
  with [#7661](https://github.com/near/nearcore/pull/7661).
  Configurable in `config.json` using `store.enable_receipt_prefetching`.

## 1.29.0 [2022-08-15]

### Protocol Changes

* Stabilized `protocol_feature_chunk_only_producers`. Validators will
  now be assigned to blocks and chunks separately.
* The validator uptime kickout threshold has been reduced to 80%
* Edge nonces between peers can now optionally indicate an expiration
  time

### Non-protocol Changes

* The logic around forwarding chunks to validators is improved
* Approvals and partial encoded chunks are now sent multiple times,
  which should reduce stalls due to lost approvals when the network is
  under high load
* We now keep a list of "TIER1" accounts (validators) for whom
  latency/reliability of messages routed through the network is
  critical
* /debug HTTP page has been improved
* Messages aren't routed through peers that are too far behind
* Log lines printed every 10 seconds are now less expensive to compute
* message broadcasts have been improved/optimized
* `network.external_address` field in config.json file is deprecated. In
  fact it has never been used and only served to confuse everyone
  [#7300](https://github.com/near/nearcore/pull/7300)
* Due to increasing state size, improved shard cache for Trie nodes to
  put more nodes in memory. Requires 3 GB more RAM
  [#7429](https://github.com/near/nearcore/pull/7429)

## 1.28.0 [2022-07-27]

### Protocol Changes

* Stabilized `alt_bn128_g1_multiexp`, `alt_bn128_g1_sum`, `alt_bn128_pairing_check` host functions [#6813](https://github.com/near/nearcore/pull/6813).

### Non-protocol Changes

* Added `path` option to `StoreConfig` which makes location to the
  RocksDB configurable via `config.json` file (at `store.path` path)
  rather than being hard-coded to `data` directory in neard home
  directory [#6938](https://github.com/near/nearcore/pull/6938)
* Removed `testnet` alias for `localnet` command; it’s been deprecated
  since 1.24 [#7033](https://github.com/near/nearcore/pull/7033)
* Removed undocumented `unsafe_reset_all` and `unsafe_reset_data`
  commands; they were deprecated since 1.25
* Key files can use `private_key` field instead of `secret_key` now;
  this improves interoperability with near cli which uses the former
  name [#7030](https://github.com/near/nearcore/issues/7030)
* Latency of network messages is now measured
  [#7050](https://github.com/near/nearcore/issues/7050)


## 1.27.0 [2022-06-22]

### Protocol Changes

* Introduced protobuf encoding as the new network protocol. Borsh support will be removed in two releases as per normal protocol upgrade policies [#6672](https://github.com/near/nearcore/pull/6672)

### Non-protocol Changes

* Added `near_peer_message_received_by_type_bytes` [#6661](https://github.com/near/nearcore/pull/6661) and `near_dropped_message_by_type_and_reason_count` [#6678](https://github.com/near/nearcore/pull/6678) metrics.
* Removed `near_<msg-type>_{total,bytes}` [#6661](https://github.com/near/nearcore/pull/6661), `near_<msg-type>_dropped`, `near_drop_message_unknown_account` and `near_dropped_messages_count` [#6678](https://github.com/near/nearcore/pull/6678) metrics.
* Added `near_action_called_count` metric [#6679]((https://github.com/near/nearcore/pull/6679)
* Removed `near_action_<action-type>_total` metrics [#6679]((https://github.com/near/nearcore/pull/6679)
* Added `near_build_info` metric which exports neard’s build information [#6680](https://github.com/near/nearcore/pull/6680)
* Make it possible to update logging at runtime: [#6665](https://github.com/near/nearcore/pull/6665)
* Use correct cost in gas profile for adding function call key [#6749](https://github.com/near/nearcore/pull/6749)

## 1.26.0 [2022-05-18]

### Protocol Changes

* Enable access key nonce range for implicit accounts to prevent tx hash collisions [#5482](https://github.com/near/nearcore/pull/5482)
* Include `promise_batch_action_function_call_weight` host function on the runtime [#6285](https://github.com/near/nearcore/pull/6285) [#6536](https://github.com/near/nearcore/pull/6536)
* Increase deployment cost [#6397](https://github.com/near/nearcore/pull/6397)
* Limit the number of locals per contract to 1_000_000
* Ensure caching all nodes in the chunk for which touching trie node cost was charged, reduce cost of future reads in a chunk [#6628](https://github.com/near/nearcore/pull/6628)
* Lower storage key limit to 2 KiB

### Non-protocol Changes

* Switch to LZ4+ZSTD compression from Snappy in RocksDB [#6365](https://github.com/near/nearcore/pull/6365)
* Moved Client Actor to separate thread - should improve performance [#6333](https://github.com/near/nearcore/pull/6333)
* Safe DB migrations using RocksDB checkpoints [#6282](https://github.com/near/nearcore/pull/6282)
* [NEP205](https://github.com/near/NEPs/issues/205): Configurable start of protocol upgrade voting [#6309](https://github.com/near/nearcore/pull/6309)
* Make max_open_files and col_state_cache_size parameters configurable [#6584](https://github.com/near/nearcore/pull/6584)
* Make RocksDB block_size configurable [#6631](https://github.com/near/nearcore/pull/6631)
* Increase default max_open_files RocksDB parameter from 512 to 10k [#6607](https://github.com/near/nearcore/pull/6607)
* Use kebab-case names for neard subcommands to make them consistent with flag names.  snake_case names are still valid for existing subcommands but kebab-case will be used for new commands.

## 1.25.0 [2022-03-16]

### Protocol Changes
* `max_gas_burnt` has been increased to 300.

### Non-protocol Changes
* More Prometheus metrics related to epoch, sync state, node version, chunk fullness and missing chunks have been added.
* Progress bar is now displayed when downloading `config.json` and `genesis.json`.
* Status line printed in logs by `neard` is now more descriptive.
- `view_state` is now a command of `neard`; `state-viewer` is no longer a separate binary.
- `RUST_LOG` environment variable is now correctly respected.
- `NetworkConfig::verify` will now fail if configuration is invalid rather than printing error and continuing.
- Fixed a minor bug which resulted in DB Not Found errors when requesting chunks.
- Updated to wasmer-near 2.2.0 which fixes a potential crash and improves cost estimator working.
- `neard init` will no longer override node or validator private keys.
- Rosetta RPC now populates `related_transactions` field.
- Rosetta RPC support is now compiled in by default. The feature still needs to be explicitly turned on and is experimental.
- Rosetta RPC /network/status end point correctly works on non-archival nodes.
- `unsafe_reset_all` and `unsafe_reset_data` commands are now deprecated. Use `rm` explicitly instead.

## 1.24.0 [2022-02-14]

### Protocol Changes

* Enable access key nonce range for implicit accounts to prevent tx hash collisions.
* Upgraded our version of pwasm-utils to 0.18 -- the old one severely under-counted stack usage in some cases.

### Non-protocol Changes

* Fix a bug in chunk requesting where validator might request chunks even if parent block hasn’t been processed yet.
* Fix memory leak in near-network.
* Change block sync to request 5 blocks at a time
* Change NUM_ORPHAN_ANCESTORS_CHECK to 3

## 1.23.0 [2021-12-13]

### Protocol Changes

* Further lower regular_op_cost from 2_207_874 to 822_756.
* Limit number of wasm functions in one contract to 10_000. [#4954](https://github.com/near/nearcore/pull/4954)
* Add block header v3, required by new validator selection algorithm
* Move to new validator selection and sampling algorithm. Now we would be able to use all available seats. First step to enable chunk only producers.

### Non-protocol Changes

* Increase RocksDB cache size to 512 MB for state column to speed up blocks processing [#5212](https://github.com/near/nearcore/pull/5212)

## 1.22.0 [2021-11-15]

### Protocol Changes
* Upgrade from Wasmer 0 to Wasmer 2, bringing better performance and reliability. [#4934](https://github.com/near/nearcore/pull/4934)
* Lower regular_op_cost (execution of a single WASM instruction) from 3_856_371 to 2_207_874. [#4979](https://github.com/near/nearcore/pull/4979)
* Lower data receipt cost and base cost of `ecrecover` host function.
* Upgrade from one shard to four shards (Simple Nightshade Phase 0)

## 1.21.0 [2021-09-06]

### Protocol Changes

* Fix some receipts that were stuck previously due to #4228. [#4248](https://github.com/near/nearcore/pull/4248)

### Non-protocol Changes

* Improve contract module serialization/deserialization speed by 30% [#4448](https://github.com/near/nearcore/pull/4448)
* Make `AccountId` strictly typed and correct by construction [#4621](https://github.com/near/nearcore/pull/4621)
* Address test dependency issue #4556 [#4606](https://github.com/near/nearcore/pull/4606). [#4622](https://github.com/near/nearcore/pull/4622).
* Fix neard shutdown issue [#4429](https://github.com/near/nearcore/pull/4429). #[4442](https://github.com/near/nearcore/pull/4442)

## 1.20.0 [2021-07-26]

### Protocol Changes

* Introduce new host functions `ecrecover` and `ripemd160`. [#4380](https://github.com/near/nearcore/pull/4380)
* Make `Account` a versioned struct. [#4089](https://github.com/near/nearcore/pull/4089)
* Limit the size of transactions to 4MB. [#4107](https://github.com/near/nearcore/pull/4107)
* Cap maximum gas price to 20x of minimum gas price. [#4308](https://github.com/near/nearcore/pull/4308), [#4382](https://github.com/near/nearcore/pull/4382)
* Fix `storageUsage` for accounts that were affected by [#3824](https://github.com/near/nearcore/issues/3824). [#4272](https://github.com/near/nearcore/pull/4274)
* Fix a bug in computation of gas for refunds. [#4405](https://github.com/near/nearcore/pull/4405)

### Non-protocol Changes

* Compile contracts after state sync. [#4344](https://github.com/near/nearcore/pull/4344)
* Introduce `max_gas_burnt_view` config for rpc. [#4381](https://github.com/near/nearcore/pull/4381)
* Fix wasmer 0.17 memory leak [#4411](https://github.com/near/nearcore/pull/4411)

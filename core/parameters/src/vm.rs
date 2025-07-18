use crate::cost::{ExtCostsConfig, ParameterCost};
use borsh::BorshSerialize;
use near_primitives_core::config::AccountIdValidityRulesVersion;
use near_primitives_core::types::Gas;
use near_schema_checker_lib::ProtocolSchema;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// NOTE that VMKind is part of serialization protocol, so we cannot remove entries from this list
// if particular VM reached publicly visible networks.
//
// Additionally, this is public only for the purposes of internal tools like the estimator. This
// API should otherwise be considered a private configuration of the `near-vm-runner`
// crate.
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    BorshSerialize,
    PartialEq,
    Eq,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
    ProtocolSchema,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum VMKind {
    /// Wasmer 0.17.x VM. Gone now.
    Wasmer0,
    /// Wasmtime VM.
    Wasmtime,
    /// Wasmer 2.x VM.
    Wasmer2,
    /// NearVM.
    NearVm,
    /// NearVM. Exists temporarily while bulk memory and reftypes are getting enabled.
    NearVm2,
}

impl VMKind {
    pub fn replace_with_wasmtime_if_unsupported(self) -> Self {
        if cfg!(not(target_arch = "x86_64")) { Self::Wasmtime } else { self }
    }
}

/// This enum represents if a storage_get call will be performed through flat storage or trie
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum StorageGetMode {
    FlatStorage,
    Trie,
}

/// Describes limits for VM and Runtime.
/// TODO #4139: consider switching to strongly-typed wrappers instead of raw quantities
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct LimitConfig {
    /// Max amount of gas that can be used, excluding gas attached to promises.
    pub max_gas_burnt: Gas,

    /// How tall the stack is allowed to grow?
    ///
    /// See <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost
    /// is calculated.
    pub max_stack_height: u32,

    /// The initial number of memory pages.
    /// NOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.
    pub initial_memory_pages: u32,
    /// What is the maximal memory pages amount is allowed to have for a contract.
    pub max_memory_pages: u32,

    /// Limit of memory used by registers.
    pub registers_memory_limit: u64,
    /// Maximum number of bytes that can be stored in a single register.
    pub max_register_size: u64,
    /// Maximum number of registers that can be used simultaneously.
    ///
    /// Note that due to an implementation quirk [read: a bug] in VMLogic, if we
    /// have this number of registers, no subsequent writes to the registers
    /// will succeed even if they replace an existing register.
    pub max_number_registers: u64,

    /// Maximum number of log entries.
    pub max_number_logs: u64,
    /// Maximum total length in bytes of all log messages.
    pub max_total_log_length: u64,

    /// Max total prepaid gas for all function call actions per receipt.
    pub max_total_prepaid_gas: Gas,

    /// Max number of actions per receipt.
    pub max_actions_per_receipt: u64,
    /// Max total length of all method names (including terminating character) for a function call
    /// permission access key.
    pub max_number_bytes_method_names: u64,
    /// Max length of any method name (without terminating character).
    pub max_length_method_name: u64,
    /// Max length of arguments in a function call action.
    pub max_arguments_length: u64,
    /// Max length of returned data
    pub max_length_returned_data: u64,
    /// Max contract size
    pub max_contract_size: u64,
    /// Max transaction size
    pub max_transaction_size: u64,
    /// Max receipt size
    pub max_receipt_size: u64,
    /// Max storage key size
    pub max_length_storage_key: u64,
    /// Max storage value size
    pub max_length_storage_value: u64,
    /// Max number of promises that a function call can create
    pub max_promises_per_function_call_action: u64,
    /// Max number of input data dependencies
    pub max_number_input_data_dependencies: u64,
    /// If present, stores max number of functions in one contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_functions_number_per_contract: Option<u64>,
    /// If present, stores max number of locals declared globally in one contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_locals_per_contract: Option<u64>,
    /// Whether to enforce account_id well-formed-ness where it wasn't enforced
    /// historically.
    #[serde(default = "AccountIdValidityRulesVersion::v0")]
    pub account_id_validity_rules_version: AccountIdValidityRulesVersion,
    /// Number of blocks after which a yielded promise times out.
    pub yield_timeout_length_in_blocks: u64,
    /// Maximum number of bytes for payload passed over a yield resume.
    pub max_yield_payload_size: u64,
    /// Hard limit on the size of storage proof generated while executing a single receipt.
    pub per_receipt_storage_proof_size_limit: usize,
}

/// Dynamic configuration parameters required for the WASM runtime to
/// execute a smart contract.
///
/// This (`VMConfig`) and `RuntimeFeesConfig` combined are sufficient to define
/// protocol specific behavior of the contract runtime. The former contains
/// configuration for the WASM runtime specifically, while the latter contains
/// configuration for the transaction runtime and WASM runtime.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Config {
    /// Costs for runtime externals
    pub ext_costs: ExtCostsConfig,

    /// Gas cost of a growing memory by single page.
    pub grow_mem_cost: u32,

    /// Gas cost of a regular operation.
    pub regular_op_cost: u32,

    /// The kind of the VM implementation to use
    pub vm_kind: VMKind,

    /// Set to `StorageGetMode::FlatStorage` in order to enable the `FlatStorageReads` protocol
    /// feature.
    pub storage_get_mode: StorageGetMode,

    /// Enable the `FixContractLoadingCost` protocol feature.
    pub fix_contract_loading_cost: bool,

    /// Enable the `ImplicitAccountCreation` protocol feature.
    pub implicit_account_creation: bool,

    /// Enable the `EthImplicitAccounts` protocol feature.
    pub eth_implicit_accounts: bool,

    /// Whether to discard custom sections.
    pub discard_custom_sections: bool,

    /// Whether to enable saturating float-to-integer wasm operators.
    pub saturating_float_to_int: bool,

    /// Whether to enable global contract related host functions.
    pub global_contract_host_fns: bool,

    /// Whether to enable saturating reference types and bulk memory wasm extensions.
    pub reftypes_bulk_memory: bool,

    /// Describes limits for VM and Runtime.
    pub limit_config: LimitConfig,
}

impl Config {
    /// Computes non-cryptographically-proof hash. The computation is fast but not cryptographically
    /// secure.
    pub fn non_crypto_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn make_free(&mut self) {
        self.ext_costs = ExtCostsConfig {
            costs: near_primitives_core::enum_map::enum_map! {
                _ => ParameterCost { gas: 0, compute: 0 }
            },
        };
        self.grow_mem_cost = 0;
        self.regular_op_cost = 0;
        self.limit_config.max_gas_burnt = u64::MAX;
    }

    pub fn enable_all_features(&mut self) {
        self.eth_implicit_accounts = true;
        self.global_contract_host_fns = true;
        self.implicit_account_creation = true;
    }
}

/// Our original code for limiting WASM stack was buggy. We fixed that, but we
/// still have to use old (`V0`) limiter for old protocol versions.
///
/// This struct here exists to enforce that the value in the config is either
/// `0` or `1`. We could have used a `bool` instead, but there's a chance that
/// our current impl isn't perfect either and would need further tweaks in the
/// future.
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
#[repr(u8)]
pub enum ContractPrepareVersion {
    /// Oldest, buggiest version.
    ///
    /// Don't use it unless specifically to support old protocol version.
    V0,
    /// Old, slow and buggy version.
    ///
    /// Better than V0, but don’t use this nevertheless.
    V1,
    /// finite-wasm 0.3.0 based contract preparation code.
    V2,
}

impl ContractPrepareVersion {
    pub fn v0() -> ContractPrepareVersion {
        ContractPrepareVersion::V0
    }
}

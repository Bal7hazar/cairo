//! Starknet capabilities and utilities on top of Cairo.
//!
//! Starknet is a smart contract platform that enables developers to build and deploy smart
//! contracts on a permissionless Layer 2 network, secured by Ethereum using validity proofs.
//!
//! Learn more at [starkware.io](http://starknet.io/).
pub mod abi;
pub mod allowed_libfuncs;
pub mod casm_contract_class;
mod compiler_version;
pub mod contract;
pub mod contract_class;
mod felt252_serde;
mod felt252_vec_compression;
pub mod plugin;
pub mod wasm_cairo_interface;

#[cfg(test)]
mod test_utils;

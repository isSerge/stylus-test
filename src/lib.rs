// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]

use alloy_primitives::Address;
use stylus_sdk::{prelude::*, storage::StorageAddress};
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        address owner;
    }
}

// private methods
impl Contract {}

// public methods
#[external]
impl Contract {
    fn get_owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }
}

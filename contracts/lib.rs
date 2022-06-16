#![cfg_attr(not(feature = "std"), no_std)]

mod cross_chain_base;

use ink_lang as ink;
pub use cross_chain_base::CrossChainBase;
pub use ink_sdk::MultiDestContracts;

#[ink::contract]
mod ink_sdk {
    use ink_prelude::string::String;

    #[ink_lang::trait_definition]
    pub trait MultiDestContracts {
        /// Returns destination contract address and action name.
        #[ink(message)]
        fn get_dest_contract_info(& self, chain_name: String) -> Option<(ink_prelude::vec::Vec<u8>, String)>;

        /// Registers destination contract to which the ink contract will send message.
        #[ink(message)]
        fn register_dest_contract(&mut self, chain_name: String, contract: String, action: String);
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct InkSdk {
    }

    impl InkSdk {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {

            }
        }

        #[ink(message)]
        pub fn placeholder(&mut self) {
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
    }
}

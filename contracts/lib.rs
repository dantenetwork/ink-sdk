#![cfg_attr(not(feature = "std"), no_std)]

pub mod cross_chain_helper;
pub use crate::ink_sdk::{
    Ownable,
    MultiDestContracts,
    CrossChainSQoS,
};

#[ink::contract]
mod ink_sdk {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use payload::message_define::{
        ISQoS,
    };

    /// This trait can be used when a contract need access control.
    #[ink::trait_definition]
    pub trait Ownable {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(& self) -> Option<AccountId>;
        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), u8>;
        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), u8>;
    }

    /// This trait can be used when a contract needs to communicate with more than one other chain.
    #[ink::trait_definition]
    pub trait MultiDestContracts {
        /// Returns destination contract address and action name.
        #[ink(message)]
        fn get_dest_contract_info(& self, chain_name: String, action: String) -> Option<(Vec<u8>, Vec<u8>)>;

        /// Registers destination contract to which the ink contract will send message.
        #[ink(message)]
        fn register_dest_contract(&mut self, chain_name: String, action: String, contract: Vec<u8>, dest_action: Vec<u8>) -> Result<(), u8>;
    }

    /// This trait can be used when a contract has custom SQoS demands.
    #[ink::trait_definition]
    pub trait CrossChainSQoS {
        /// Inserts one SQoS item.
        /// If the item exists, it will be replaced.
        #[ink(message)]
        fn set_sqos(&mut self, sqos_item: ISQoS);

        /// Removes one SQoS item.
        #[ink(message)]
        fn remove_sqos(&mut self);

        // /// Clear all SQoS items.
        // #[ink(message)]
        // fn clear(&mut self) -> Result<(), u8>;

        // /// Sets SQoS items
        // #[ink(message)]
        // fn set(&mut self, sqos: Vec<ISQoS>) -> Result<(), u8>;

        /// Returns SQoS items
        #[ink(message)]
        fn get_sqos(& self) -> Option<ISQoS>;
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

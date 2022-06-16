#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod greeting {
    use ink_sdk::{
        CrossChainBase,
        MultiDestContracts,
    };
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use payload::message_define::{
        ISentMessage,
        ISession,
        ISQoS,
        ISQoSType,
        IContent,
    };
    use payload::message_protocol::{
        MsgType,
        MessagePayload,
    };
    use ink_storage::{
        Mapping,
        traits::SpreadAllocate,
    };
    
    #[derive(::scale::Encode, ::scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        MethodNotRegisterd,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Greeting {
        cross_chain_contract: Option<AccountId>,
        ret: Option<String>,
        dest_contract_map: Mapping<String, (ink_prelude::vec::Vec<u8>, String)>,
    }

    /// We use `CrossChainBase` of SDK here, to be able to use the basic cross-chain functionalities.
    impl CrossChainBase for Greeting {
        fn get_cross_chain_contract_address(& self) -> AccountId {
            self.cross_chain_contract.unwrap()
        }
    }

    /// We use `MultiDestContracts` of SDK here, to be able to send messages to multi chains.
    impl MultiDestContracts for Greeting {      
        #[ink(message)]  
        fn get_dest_contract_info(& self, chain_name: String) -> Option<(ink_prelude::vec::Vec<u8>, String)> {
            self.dest_contract_map.get(chain_name)
        }

        #[ink(message)]
        fn register_dest_contract(&mut self, chain_name: String, contract: String, action: String) {
            let contract_bytes = Vec::<u8>::from(contract);
            self.dest_contract_map.insert(chain_name, &(contract_bytes, action));
        }
    }

    impl Greeting {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {
            })
        }

        /// Sets cross-chain contract address
        #[ink(message)]
        pub fn set_cross_chain_contract(&mut self, contract: AccountId) {
            self.cross_chain_contract = Some(contract);
        }

        /// Sends greeting to another chain 
        #[ink(message)]
        pub fn send_greeting(&mut self, chain_name: String, greeting: Vec<String>) -> Result<(), Error> {
            let dest = self.get_dest_contract_info(chain_name.clone()).ok_or(Error::MethodNotRegisterd)?;
            let contract = dest.0;
            let action = dest.1;

            let mut msg_payload = MessagePayload::new();
            msg_payload.push_item(String::try_from("greeting").unwrap(), MsgType::InkStringArray, greeting.clone());
            let data = msg_payload.to_bytes();

            let mut sqos = Vec::<ISQoS>::new();
            sqos.push(ISQoS::new(ISQoSType::Reveal, None));
            let session = ISession::new(0, 0);
            let content = IContent::new(contract, action, data);
            let message = ISentMessage::new(chain_name, sqos, content, session);

            self.send_message(message);

            Ok(())
        }

        /// Receives greeting from another chain 
        #[ink(message)]
        pub fn receive_greeting(&mut self, payload: MessagePayload) -> String {
            let item = payload.get_item(String::try_from("greeting").unwrap()).unwrap();
            let param: Vec<String> = scale::Decode::decode(&mut item.v.as_slice()).unwrap();
            // let payload
            let mut s = String::new();
            s = s + &ink_prelude::format!("{:?}", param);
            self.ret = Some(s.clone());
            s
        }

        /// Receives message from another chain 
        #[ink(message)]
        pub fn get_ret(& self) -> String {
            self.ret.clone().unwrap()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        use payload::message_define::{
            ISentMessage,
            ISession,
            ISQoS,
            IContent,
        };

        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            let locker = Greeting::new();
        }

        /// We test if set_cross_chain_contract works.
        #[ink::test]
        fn set_cross_chain_contract_works() {
            let mut locker = Greeting::new();
            let contract_id = ink_env::test::callee::<ink_env::DefaultEnvironment>();
            locker.set_cross_chain_contract(contract_id);
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

mod cross_chain_base;

use ink_lang as ink;
pub use cross_chain_base::CrossChainBase;
use Payload::message_define::ISentMessage;

#[ink::contract]
mod ink_sdk {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use Payload::message_define::{
        ISentMessage,
        ISession,
        ISQoS,
        ISQoSType,
        IContent,
    };
    use Payload::message_protocol::{
        MsgType,
        MessagePayload,
    };
    use super::CrossChainBase;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct InkSdk {
        cross_chain_contract: Option<AccountId>,
        ret: Option<String>,
    }

    impl InkSdk {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                cross_chain_contract: None,
                ret: None,
            }
        }

        /// Sets cross-chain contract address
        #[ink(message)]
        pub fn set_cross_chain_contract(&mut self, contract: AccountId) {
            self.cross_chain_contract = Some(contract);
        }

        /// Sends message to another chain 
        #[ink(message)]
        pub fn send_cross_chain_message(&mut self, chain_name: String, greeting: Vec<String>) {
            let contract = String::try_from("0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b").unwrap();
            let action = String::try_from("receiveGreeting").unwrap();

            let mut msg_payload = MessagePayload::new();
            msg_payload.push_item(String::try_from("greeting").unwrap(), MsgType::InkStringArray, greeting.clone());
            let data = msg_payload.to_bytes();

            let mut sqos = Vec::<ISQoS>::new();
            sqos.push(ISQoS::new(ISQoSType::Reveal, None));
            let session = ISession::new(0, 0);
            let content = IContent::new(contract, action, data);
            let message = ISentMessage::new(chain_name.clone(), sqos, content, session);

            self.send_message(message);
        }

        /// Receives message from another chain 
        #[ink(message)]
        pub fn receive_cross_chain_message(&mut self, payload: MessagePayload) -> String {
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

    impl CrossChainBase for InkSdk {
        /// Returns cross-chain contract address
        fn get_cross_chain_contract_address(& self) -> AccountId {
            self.cross_chain_contract.unwrap()
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
        use Payload::message_define::{
            ISentMessage,
            ISession,
            ISQoS,
            IContent,
        };

        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            let locker = InkSdk::new();
        }

        /// We test if set_cross_chain_contract works.
        #[ink::test]
        fn set_cross_chain_contract_works() {
            let mut locker = InkSdk::new();
            let contract_id = ink_env::test::callee::<ink_env::DefaultEnvironment>();
            locker.set_cross_chain_contract(contract_id);
        }
    }
}

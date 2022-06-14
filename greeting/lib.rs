#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod greeting {
    use ink_sdk::CrossChainBase;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use Payload::message_define::{
        ISentMessage,
        ISession,
        ISQoS,
        ISQoSType,
        IContent,
        IError,
    };
    use Payload::message_protocol::{
        MsgType,
        MessageItem,
        MessagePayload,
    };

    fn convert_address(s: &str) -> AccountId {
        let mut begin = 0;
        if &s[0..2] == "0x" {
            begin = 2;
        }
    
        let mut v: [u8; 32] = [0; 32];
        let mut index = 0;
        for i in begin/2..s.len()/2 {
            v[index] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
            index = index + 1;
        }
        
        AccountId::try_from(v).unwrap()
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Greeting {
        cross_chain_contract: Option<AccountId>,
        ret: Option<String>,
    }

    impl Greeting {
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

        /// Sends greeting to another chain 
        #[ink(message)]
        pub fn send_greeting(&mut self, chain_name: String, greeting: Vec<String>) {
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

    impl CrossChainBase for Greeting {
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

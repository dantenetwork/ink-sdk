#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod greeting {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink_sdk::{cross_chain_helper, CrossChainSQoS, MultiDestContracts, Ownable};
    use payload::message_define::{IContent, IContext, IRequestMessage, ISQoS};
    use payload::message_protocol::{MessagePayload, MsgDetail};

    #[derive(::scale::Encode, ::scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        MethodNotRegisterd,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    // #[derive(SpreadAllocate)]
    pub struct Greeting {
        /// Account id of owner
        owner: Option<AccountId>,
        cross_chain_contract: Option<AccountId>,
        ret: Mapping<(String, u128), String>,
        dest_contract_map: Mapping<(String, String), (Vec<u8>, Vec<u8>)>,
    }

    /// We use `CrossChainBase` here, to be able to use the sdk functionalities.
    impl cross_chain_helper::CrossChainBase for Greeting {
        fn get_cross_chain_contract_address(&self) -> AccountId {
            self.cross_chain_contract.unwrap()
        }
    }

    /// We need access control.
    impl Ownable for Greeting {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.owner
        }

        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), u8> {
            self.only_owner()?;

            self.owner = None;

            Ok(())
        }

        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), u8> {
            self.only_owner()?;

            self.owner = Some(new_owner);

            Ok(())
        }
    }

    /// We use `MultiDestContracts` of SDK here, to be able to send messages to multi chains.
    impl MultiDestContracts for Greeting {
        #[ink(message)]
        fn get_dest_contract_info(
            &self,
            chain_name: String,
            action: String,
        ) -> Option<(Vec<u8>, Vec<u8>)> {
            self.dest_contract_map.get((chain_name, action))
        }

        #[ink(message)]
        fn register_dest_contract(
            &mut self,
            chain_name: String,
            action: String,
            contract: Vec<u8>,
            dest_action: Vec<u8>,
        ) -> Result<(), u8> {
            self.only_owner()?;

            self.dest_contract_map
                .insert((chain_name, action), &(contract, dest_action));

            Ok(())
        }
    }

    /// We use `CrossChainSQoS` here, because
    impl CrossChainSQoS for Greeting {
        /// Inserts one SQoS item.
        /// If the item exists, it will be replaced.
        #[ink(message)]
        fn set_sqos(&mut self, sqos_item: ISQoS) {
            // self.only_owner()?;

            let account_id = Self::env().account_id();
            cross_chain_helper::set_sqos(self, sqos_item, account_id);
        }

        /// Removes one SQoS item.
        #[ink(message)]
        fn remove_sqos(&mut self) {
            // self.only_owner()?;

            let account_id = Self::env().account_id();
            if let Some(_) = cross_chain_helper::get_sqos(self, account_id) {
                cross_chain_helper::remove_sqos(self, account_id);
            }
        }

        /// Returns SQoS items
        #[ink(message)]
        fn get_sqos(&self) -> Option<ISQoS> {
            let account_id = Self::env().account_id();
            cross_chain_helper::get_sqos(self, account_id)
        }
    }

    impl Greeting {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Some(Self::env().caller()),
                cross_chain_contract: None,
                ret: Default::default(),
                dest_contract_map: Default::default(),
            }
        }

        /// Initializes the contract with the specified chain name.
        // fn new_init(&mut self) {
        //     let caller = Self::env().caller();
        //     self.owner = Some(caller);
        // }

        /// Sets cross-chain contract address
        #[ink(message)]
        pub fn set_cross_chain_contract(&mut self, contract: AccountId) {
            // self.only_owner()?;

            self.cross_chain_contract = Some(contract);

            // Ok(())
        }

        #[ink(message)]
        pub fn clear_ret(&mut self, chain_name: String, id: u128) -> Result<(), u8>{
            self.only_owner()?;
            self.ret.remove(&(chain_name, id));
            Ok(())
        }

        /// If caller is the owner of the contract
        fn only_owner(&self) -> Result<(), u8> {
            let caller = self.env().caller();
            if self.owner.unwrap() != caller {
                return Err(1);
            }

            Ok(())
        }

        /// Sends greeting to another chain
        #[ink(message)]
        pub fn send_greeting(
            &mut self,
            chain_name: String,
            greeting: Vec<String>,
        ) -> Result<(), Error> {
            let dest = self
                .get_dest_contract_info(
                    chain_name.clone(),
                    String::try_from("receive_greeting").unwrap(),
                )
                .ok_or(Error::MethodNotRegisterd)?;
            let contract = dest.0;
            let action = dest.1;

            let mut msg_payload = MessagePayload::new();
            msg_payload.push_item(
                String::try_from("greeting").unwrap(),
                MsgDetail::InkStringArray(greeting.clone()),
            );
            let data = msg_payload.to_bytes();

            let sqos = Vec::<ISQoS>::new();
            // sqos.push(ISQoS::new(ISQoSType::Reveal, Vec::new()));
            let content = IContent::new(contract, action, data);
            let message = IRequestMessage::new(chain_name, sqos, content);

            cross_chain_helper::cross_chain_send_message(self, message);

            Ok(())
        }

        /// Receives greeting from another chain
        #[ink(message)]
        pub fn receive_greeting(&mut self, payload: MessagePayload) -> String {
            let item = payload
                .get_item(String::try_from("greeting").unwrap())
                .unwrap();
            // let param: Vec<String> = scale::Decode::decode(&mut item.v.as_slice()).unwrap();
            let param = item.in_to::<Vec<String>>();
            let context: IContext = cross_chain_helper::get_context(self).unwrap();
            // let payload
            let mut s = String::new();
            s = s + &ink::prelude::format!("{:?}", param);
            self.ret.insert((context.from_chain, context.id), &s);
            s
        }

        /// Receives message from another chain
        #[ink(message)]
        pub fn get_ret(&self, key: (String, u128)) -> String {
            self.ret.get(key).unwrap_or(String::from("No value"))
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if set_cross_chain_contract works.
        #[ink::test]
        fn set_cross_chain_contract_works() {
            let mut locker = Greeting::new();
            let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();
            locker.set_cross_chain_contract(contract_id);
        }
    }
}

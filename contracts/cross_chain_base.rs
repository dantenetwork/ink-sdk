use ink_env::AccountId;

use crate::ISentMessage;

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

const CROSS_CHAIN_CONTRACT_ADDRESS: &str = "0x0b8721b619a14ac6134676db96a8d5e30c92a3456cca77d2dd273167f9621f0f";
const SEND_MESSAGE_SELECTOR: [u8; 4] = [0x27, 0x26, 0x79, 0x17];

pub trait CrossChainBase {
    /// Returns cross-chain contract address
    fn get_cross_chain_contract_address(& self) -> AccountId {
        let default_address = convert_address(CROSS_CHAIN_CONTRACT_ADDRESS);
        default_address
    }

    /// Send cross-chain message
    fn send_message(& self, message: ISentMessage) -> u128 {
        let cross_chain: AccountId = self.get_cross_chain_contract_address();
        
        let id: u128 = ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(cross_chain)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new(SEND_MESSAGE_SELECTOR))
                    .push_arg(message)
                )
                .returns::<u128>()
                .fire()
                .unwrap();
        id
    }
}
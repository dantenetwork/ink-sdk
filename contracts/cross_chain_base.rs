use ink_env::AccountId;

use payload::message_define::ISentMessage;

/// Converts hex string of address into [u8; 32]
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

const CROSS_CHAIN_CONTRACT_ADDRESS: &str = "0x9b33e9dbcc468833b9cec8e0642e4932487931ea092d789ffe51ee41fea4de7a";
const SEND_MESSAGE_SELECTOR: [u8; 4] = [0x27, 0x26, 0x79, 0x17];

pub trait CrossChainBase {
    /// Returns cross-chain contract address.
    /// If you do not want to use the built-in address, you can rewrite this method.
    fn get_cross_chain_contract_address(& self) -> AccountId {
        let default_address = convert_address(CROSS_CHAIN_CONTRACT_ADDRESS);
        default_address
    }

    /// Sends cross-chain message, and returns message id.
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
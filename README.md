# Ink! SDK
Ink! SDK makes it easy for ink developers in the Polkadot Ecosystem to use Dante Network to interact with other chains, such as Near, Ethereum, Avalanche, Flow.

## Library
The library witch is contained in `contracts` is used to develop Ink! application contracts. The library provides two functional modules, practical traits and cross-chain interacting module.

### Practical Traits
#### MultiDestContracts
This trait can be used when a contract needs to communicate with more than one other chain.

#### CrossChainSQoS
This trait can be used when a contract has custom SQoS demands.

### Cross-chain Interacting Module
The cross-chain interaction module is contained in the file `cross_chain_helper.rs`, which mainly provides functions to make cross-contract calls to cross-chain contract, as well as to make cross-chain interaction with other chains.

#### cross_chain_send_message
`cross_chain_send_message` sends a cross-chain message, and returns the message id recorded in the cross-chain contract.

Example is shown below, or you can refer it in the example greeting.
```
#[ink(message)]
pub fn send_greeting(&mut self, chain_name: String, greeting: Vec<String>) -> Result<(), Error> {
    ...
    let message = IRequestMessage::new(chain_name, sqos, content);

    cross_chain_helper::cross_chain_send_message(self, message);

    Ok(())
}
```

#### cross_chain_call
`cross_chain_call` sends a cross-chain message, and returns the message id recorded in the cross-chain contract. Later a callback in the application contract will be called.

Example is shown below, or you can refer it in the example osComputing.
```
#[ink(message)]
pub fn send_computing_task(&mut self, chain_name: String, nums: Vec<u32>) -> Result<(), Error> {
    ...
    let message = IRequestMessage::new(chain_name, sqos, content);

    cross_chain_helper::cross_chain_call(self, message);

    Ok(())
}
```

#### cross_chain_respond
`cross_chain_respond` responds a cross-chain request, and returns the message id recorded inthe cross-chain contract.

Example is shown below, or you can refer it in the example osComputing.
```
/// Receives computing task from another chain 
#[ink(message)]
pub fn receive_computing_task(&mut self, payload: MessagePayload) -> String {
    ...
    let message = IResponseMessage::new(sqos, content);
    cross_chain_helper::cross_chain_respond(self, message);

    String::try_from("Ok").unwrap()
}
```

## examples
There are two examples in the repo, one is `greeting`, the other is `osComputing`. You can use either of the both examples as a template, if you want to create a new project.

### greeting
The example shows how to send greeting messages to, and receive greeting messages from other chains with the Ink! SDK.

### osComputing
The example shows a scenario in which sb. want to send a outsource computing task to another chain, and receive the result.
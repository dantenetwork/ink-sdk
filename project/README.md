# Start your project here

## Clone `message_ink` to your local
```bash
git clone https://github.com/dantenetwork/ink-sdk
```

## Create a New project

```bash
cargo contract new <your project name>
```

## Add `message_ink` library into `Cargo.toml`, like following:
```rust
payload = { path = "../../../message-ink/payload/", default-features = false, features = ["ink-as-dependency"] }
```

This will be published on `crates.io` later.

## Add `ink_sdk` library into `Cargo.toml`, like following:
```rust
ink_sdk = { path = "../../contracts/", default-features = false, features = ["ink-as-dependency"] }
```


## More details
* [Official tutorial](https://docs.substrate.io/tutorials/smart-contracts/first-smart-contract/)
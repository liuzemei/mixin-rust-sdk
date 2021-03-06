# mixin-sdk
[中文版](./README.zh-CN.md)
The rust sdk of mixin.one.
Currently, only clients generated by RSA's keystore are supported.

## Installation
1. Add dependencies to the `[dependencies]` of `Cargo.toml` in the project
```toml
[dependencies]
mixin-sdk = "0.0.2"
```

2. The terminal executes `cargo build` to complete the installation
## Usage
```rust
use mixin_sdk::{keystore::Keystore, Client};

async fn test() -> Result<(), Box<std::error::Error>> {
    let ks = Keystore::new(Keystore {
        client_id: String::from(""),
        client_secret: String::from(""),
        session_id: String::from(""),
        private_key: String::from(""),
        pin_token: String::from(""),
        pin: String::from(""),
        scope: String::from("FULL"),
    });
    let client = Client::new(ks);
    let user = client.user_me().await?;
    println!(
        "user_id:{}\nfull_name:{}\nidentity_number:{}\n",
        user.user_id, user.full_name, user.identity_number
    );
    Ok(())
}
```



## Example
1. In order to see the execution effect in main.rs, we introduce another package.
```toml
[dependencies]
mixin-sdk = "0.0.2"
tokio = {version = "1.2.0", features = ["full"]}
```

2. We write the following code in `main.rs`
```rust
use mixin_sdk::{keystore::Keystore, Client};
#[tokio::main]
async fn main() {
    test().await;
}
async fn test() -> Result<(), Box<std::error::Error>> {
    let ks = Keystore::new(Keystore {
        client_id: String::from(""),
        client_secret: String::from(""),
        session_id: String::from(""),
        private_key: String::from(""),
        pin_token: String::from(""),
        pin: String::from(""),
        scope: String::from("FULL"),
    });
    let client = Client::new(ks);
    let user = client.user_me().await?;
    println!(
        "user_id:{}\nfull_name:{}\nidentity_number:{}\n",
        user.user_id, user.full_name, user.identity_number
    );
    Ok(())
}
```
3. Complete the filling of `client_id`/`client_secret`/`session_id`/`private_key`/`pin_token`/`pin`. If the scope has no special requirements, fill in `FULL`
4. Execute `cargo run` in the terminal
5. If the above information is filled in correctly, your robot information should appear on the screen.
> If you have any questions, please issue an issue.


## Contribute

Acceptable PRs.


## Related articles or links
> 1. [https://developers.mixin.one/document](https://developers.mixin.one/document)
> 2. [https://github.com/fox-one/mixin-sdk-go](https://github.com/fox-one/mixin-sdk-go)
> 3. [https://mixin.one](https://mixin.one)



## License

MIT © Richard McRichface
# mixin-sdk
mixin.one 的 rust sdk.
目前只支持了 RSA 的 keystore 生成的客户端.

# 安装
1. 在项目中的  `Cargo.toml` 的 `[dependencies]` 中加入依赖
```toml
[dependencies]
mixin-sdk = "0.0.2"
```

2. 终端执行 `cargo build` 即可完成安装
# 使用
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



# 例子
1. 为了在 main.rs 中看到执行效果. 我们引入另外一个包.
```toml
[dependencies]
mixin-sdk = "0.0.2"
tokio = { version = "1.2.0", features = ["full"] }
```

2. 我们在 `main.rs` 中编写如下代码
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
3. 完成对 `client_id`/`client_secret`/`session_id`/`private_key`/`pin_token`/`pin` 的填写, scope 没有特殊要求的话, 就填写 `FULL`
4. 在终端执行 `cargo run`
5. 如果上述信息均填写正确的话, 屏幕中应该会出现您的机器人信息.
> 如果有问题, 欢迎 issue.



## 贡献

可接受 PRs.


## 相关文章或链接
> 1. [https://developers.mixin.one/document](https://developers.mixin.one/document)
> 2. [https://github.com/fox-one/mixin-sdk-go](https://github.com/fox-one/mixin-sdk-go)
> 3. [https://mixin.one](https://mixin.one)



## License

MIT © Richard McRichface




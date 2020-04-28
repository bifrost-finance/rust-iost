# rust-iost

An Iost library implemented in Rust

## Execute test case

1. Install RUST

```
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone rust-iost repository

```
# git clone https://github.com/bifrost-codes/rust-iost.git
```

3. Run Cargo test

```
# cargo test
# test result: SUCCESSED. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Example

lib:

~~~rust
async fn get<T>(&self, path: &str) -> Result<T, Error> where T: 'static + for<'de>Deserialize<'de> {
        let url = format!("{}/{}", self.host, path);
        let response = self.client.get(&url).send().await.map_err(Error::Reqwest)?;
        if response.status() == 200 {
            let result = response.json::<T>().await.map_err(Error::Reqwest)?;
            Ok(result)
        } else {
            let rsp = response.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
            Err(Error::ErrorMessage(rsp))
        }
    }
~~~

~~~rust
//Execute test file command "cargo test iost_basic_test -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn iost_basic_test() {
        let host = "https://api.iost.io";
        let iost = IOST::new(host);
        let result = iost.get_node_info().await;
        assert!(result.is_ok());
        let chain_result = iost.get_chain_info().await;
        assert!(chain_result.is_ok());
        let gas_result = iost.get_gas_ratio().await;
        assert!(gas_result.is_ok());
        let ram_result = iost.get_ram_info().await;
        assert!(ram_result.is_ok());
    }
}
~~~

get_token_balance:

~~~rust
async fn get_token_balance(domain: &str, account: &str, token: &str, by_longest_chain: bool) -> Result<TokenBalance, Error> {
    let url = format!("{}/getTokenBalance/{}/{}/{}", domain, account, token, by_longest_chain);
    let req = reqwest::get(&url).await.map_err(Error::Reqwest)?;
    if req.status() == 200 {
        let rsp = req.json::<TokenBalance>().await.map_err(Error::Reqwest)?;
        Ok(rsp)
    } else {
        let rsp = req.json::<ErrorMessage>().await.map_err(Error::Reqwest)?;
        Err(Error::ErrorMessage(rsp))
    }
}
~~~

~~~rust
//Execute test file command "cargo test get_token_balance_should_be_ok -- --nocapture"
#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_token_balance_should_be_ok() {
        let response = get_token_balance("http://api.iost.io","admin","iost",true).await;
        assert!(response.is_ok());
    }
}
~~~


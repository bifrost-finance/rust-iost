use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct GasRatio {
    /// the lowest gas ratio of the most recently packed blocks
    pub lowest_gas_ratio: f64,
    /// the median gas ratio of the most recently packed blocks
    pub median_gas_ratio: f64
}

async fn get_gas_ratio() -> GasRatio{
    let res = reqwest::get("https://api.iost.io/getGasRatio").await.unwrap()
        .json::<GasRatio>()
        .await.unwrap();
    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_gas_ratio_should_be_ok() {

        println!("{:#?}", get_gas_ratio().await);

    }
}

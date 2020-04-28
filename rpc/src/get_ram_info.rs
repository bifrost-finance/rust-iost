use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct RamInfo {
    /// RAM available, in byte
    pub available_ram: String,
    /// The amount of RAM sold, in byte
    pub used_ram: String,
    /// The system's total RAM count, in byte
    pub total_ram: String,
    /// The buying price of RAM, in IOST/byte
    pub buy_price: f64,
    /// 	The selling price of RAM, in IOST/byte
    pub sell_price: f64
}

async fn get_ram_info() -> RamInfo {
    let res = reqwest::get("https://api.iost.io/getRAMInfo").await.unwrap()
        .json::<RamInfo>()
        .await.unwrap();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_ram_info_should_be_ok() {

        println!("{:#?}", get_ram_info().await);

    }
}

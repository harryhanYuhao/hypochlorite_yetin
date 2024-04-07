mod scrape;
mod web_driver;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let _driver = web_driver::initialize_driver(true).await?;
    // https://huaweiuk.teamtailor.com/jobs
    scrape::huawei::scrape_huwei_job().await?;
    Ok(())
}

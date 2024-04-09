mod scrape;
mod web_driver;
use colored::Colorize;
use serde::Serialize;
use std::error::Error;
use std::sync::Mutex;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use hypochlorite::JobEntry;
use hypochlorite::CONFIG;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    hypochlorite::init()?;
    // killguard has custom drop traits that kill the subprocess
    // It has to be declared in main
    let _kill_guard = web_driver::KillChildGuard;
    let driver = web_driver::initialize_driver(false).await?;
    scrape::short_pause();
    scrape::huawei::scrape(&driver).await?;
    scrape::amd::scrape(&driver).await?;
    driver.quit().await?;
    Ok(())
}

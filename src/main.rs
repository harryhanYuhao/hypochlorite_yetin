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

#[derive(Debug, Serialize, Default)]
pub struct JobEntry {
    // company_name: String,
    company_name: String,
    job_title: String,
    apply_link: String,
    job_type_time: String,
    is_rolling: bool,
    location: String,
    ddl: String,
    start_time: String,
    duration: String,
    salary: String,
    description: String,
    keyworkds: String,
}

/// Init function check:
/// 1: if directory data exists, and create it if not
/// 2: if chromdriver is in the root directory, and panic if not
fn init() -> Result<(), Box<dyn Error>> {
    static HAS_RUN: Mutex<Option<bool>> = Mutex::new(Some(false));
    let mut has_run = HAS_RUN.lock().unwrap();
    {
        if *has_run.as_ref().unwrap() {
            println!("init() Already Run!!! This function shall only be called once");
            return Err("Bad!".into());
        }
    }
    *has_run = Some(true);
    if !std::path::Path::new("data").exists() {
        std::fs::create_dir("data")?;
    }
    if !std::path::Path::new("chromedriver").exists() {
        panic!(
            "{}\n{}\n{}\n{}",
            "Chrome Driver does not exist!",
            "Download The Chrome Driver!".red().bold(),
            "This in unrecoverable error.",
            "Please Download the Chrome Driver with the same version as your browser. See readme.md"
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    // killguard has custom drop traits that kill the subprocess
    // It has to be declared in main
    let _kill_guard = web_driver::KillChildGuard;
    let driver = web_driver::initialize_driver(false).await?;
    scrape::short_pause();
    // scrape::huawei::scrape_huawei_job(&driver).await?;
    scrape::amd::scrape_amd_job(&driver).await?;
    // driver.quit().await?;
    Ok(())
}

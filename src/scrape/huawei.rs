use crate::web_driver::initialize_driver;

use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use url::Url;

pub async fn scrape_huwei_job() -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver(true).await?;
    let url = Url::parse("https://huaweiuk.teamtailor.com/jobs")?;

    driver.goto(url).await?;
    thread::sleep(Duration::from_secs(2));

    // search_location(&driver, place).await?;
    thread::sleep(Duration::from_secs(1));
    click_popup(&driver).await?;

    // scrape_all(driver).await?;

    Ok(())
}

async fn click_popup(driver: &WebDriver) -> Result<(), Box<dyn Error>>{
    let popup_menu_ok_button = driver
        .find(By::XPath("/html/body/dialog[1]/div[2]/button[1]"))
        .await?;
    popup_menu_ok_button.wait_until().clickable().await?;
    popup_menu_ok_button.click().await?;
    Ok(())
}



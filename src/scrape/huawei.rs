//! This module provide scrape_huawei_job() function
//! which scrapes jobs posted on "https://huaweiuk.teamtailor.com/jobs" (jobs in uk)
//! and save the result to "data/huawei_uk.csv"

use crate::JobEntry;
use colored::Colorize;
use std::error::Error;
use std::fs::OpenOptions;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, WebDriver, WebElement,
};
use url::Url;

pub async fn scrape(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    let save_to = format!(
        "{}huawei_gb.csv",
        crate::CONFIG.lock().unwrap().raw_data_dir
    );
    scrape_site_uk(
        driver,
        "https://huaweiuk.teamtailor.com/jobs",
        save_to.as_str(),
    )
    .await
}

// for serializing to csv
async fn job_entry_from_element(element: &WebElement) -> Result<JobEntry, WebDriverError> {
    let title = element.find(By::Css("a > span")).await?.text().await?;
    let url = element
        .find(By::Css("a"))
        .await?
        .attr("href")
        .await?
        .unwrap_or_default();
    Ok(JobEntry {
        job_title: title,
        apply_link: url,
        ..Default::default() // default defined in main
    })
}

async fn scrape_site_uk(
    driver: &WebDriver,
    url: &str,
    save_to: &str,
) -> Result<(), Box<dyn Error>> {
    let url_tmp = Url::parse(url)?;
    driver.goto(url_tmp).await?;
    println!("{} at {}", "Scraping Huawei job".yellow().bold(), url);
    super::short_pause();

    // clicking the popup menu of cookies permission
    click_popup(driver).await?;
    super::short_pause();

    // clicking all the "more job" buttons, if any
    // So that all entries are displayed
    click_more_job_button(driver).await?;
    let mut wtr = csv::Writer::from_path(save_to)?;
    println!("Writing to {}", save_to);
    let all_entry = get_all_entry(driver).await?;
    for entry in all_entry {
        let mut tmp = job_entry_from_element(&entry).await?;
        tmp.location = "UK".to_string();
        tmp.company_name = "Huawei".to_string();
        wtr.serialize(tmp)?;
    }
    Ok(())
}

/// There will be popup menu asking for cookies permission
/// This function will click "only allow necessary cookies"
async fn click_popup(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    if let Ok(popup_menu_ok_button) = driver
        .find(By::XPath("/html/body/dialog[1]/div[2]/button[2]"))
        .await
    {
        popup_menu_ok_button.wait_until().clickable().await?;
        popup_menu_ok_button.click().await?;
        return Ok(());
    }
    println!("No popup menu found; continuing...");
    return Ok(());
}

async fn click_more_job_button(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    loop {
        if let Ok(more_job_button) = driver
            .find(By::XPath(
                "/html/body/main/section/turbo-frame/div/div/div/div[2]/div[3]/a/span/span",
            ))
            .await
        {
            super::scroll_into_view(driver, &more_job_button).await?;
            println!("Pausing");
            super::long_pause();
            println!("finished pausing, waiting for more job button to be clickable");
            more_job_button.wait_until().clickable().await?;
            println!("More job button is clickable");
            more_job_button.click().await?;
            println!("More job button clicked");
            super::short_pause();
        } else {
            break;
        }
    }
    Ok(())
}

async fn get_all_entry(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    driver.find_all(By::Css("#jobs_list_container > li")).await
}

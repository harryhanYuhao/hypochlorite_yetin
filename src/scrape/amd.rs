//! This module provide scrape_amd_job() function
//! which scrapes jobs posted on
//! "https://careers.amd.com/careers-home/jobs?page=1&location=china%20&woe=12&stretchUnit=MILES&stretch=10&sortBy=relevance"
//! (jobs in china)
//! and save the result to "data/huawei_uk.csv"

use super::{long_pause, medium_pause, scroll_to_bottom, short_pause};
use crate::JobEntry;
use colored::Colorize;
use std::error::Error;
use std::fs::OpenOptions;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, WebDriver, WebElement,
};
use url::Url;

struct AmdUrl {
    country_code: String,
    url: String,
}

async fn job_entry_from_elementid(
    driver: &WebDriver,
    id: usize,
) -> Result<JobEntry, Box<dyn Error>> {
    let title_selector = format!("#link-job-{} > span", id);
    let link_selector = format!("#link-job-{}", id);
    let title = driver.find(By::Css(&title_selector)).await?.text().await?;
    println!("Title: {}", title);
    let url = driver
        .find(By::Css(&link_selector))
        .await?
        .attr("href")
        .await?
        .unwrap_or_default();
    // the site only provides relative url: change it to absolute
    let url = format!("https://careers.amd.com{url}");
    Ok(JobEntry {
        job_title: title,
        apply_link: url,
        ..Default::default() // default defined in main
    })
}

pub async fn scrape_amd_job(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    let amd_urls = vec![
        AmdUrl{
            country_code: "CN".to_string(),
            url: "https://careers.amd.com/careers-home/jobs?page=1&location=china%20&woe=12&stretchUnit=MILES&stretch=10&sortBy=relevance&limit=100".to_string(),
        },
        AmdUrl{
            country_code: "GB".to_string(),
            url: "https://careers.amd.com/careers-home/jobs?page=1&location=united%20kingdom&woe=1&stretchUnit=MILES&stretch=10&sortBy=relevance&limit=100".to_string(),
        },
    ];
    scrape_site(
        driver,
        "https://careers.amd.com/careers-home/jobs?page=1&location=china%20&woe=12&stretchUnit=MILES&stretch=10&sortBy=relevance&limit=100",
        "data/amd_cn.csv",
    )
    .await
}

async fn scrape_site(
    driver: &WebDriver,
    url: &str,
    save_to: &str,
) -> Result<(), Box<dyn Error>> {
    let url_tmp = Url::parse(url)?;
    driver.goto(url_tmp).await?;
    println!("{} at {}", "Scraping AMD job".yellow().bold(), url);
    long_pause();

    // clicking the popup menu of cookies permission
    click_popup(driver).await?;
    short_pause();

    //
    scroll_to_bottom(driver).await?;
    short_pause();
    let mut wtr = csv::Writer::from_path(save_to)?;
    record_entry_to_csv(driver, &mut wtr).await?;
    while next_page_button_exists_or_clickable(driver).await? {
        long_pause();
        click_next_page_button(driver).await?;
        medium_pause();
        scroll_to_bottom(driver).await?;
        short_pause();
        record_entry_to_csv(driver, &mut wtr).await?;
    }

    Ok(())
}

/// There will be popup menu asking for cookies permission
/// This function will click "only allow necessary cookies"
async fn click_popup(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    if let Ok(popup_menu_ok_button) = driver
        .find(By::XPath(
            "/html/body/div[7]/div[2]/div/div[1]/div/div[2]/div/button[2]",
        ))
        .await
    {
        println!("Popup menu found; waiting for it to be clickable");
        popup_menu_ok_button.wait_until().clickable().await?;
        println!("Popup menu is clickable; clicking it");
        popup_menu_ok_button.click().await?;
        return Ok(());
    }
    println!("No popup menu found; continuing...");
    return Ok(());
}

async fn click_next_page_button(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
    if let Ok(next_page_button) = driver
            .find(By::XPath(
                "/html/body/div[2]/search-app/search-base-search-holder/search-results/div/search-job-paginator/mat-paginator/div/div/div[2]/button[2]"
            ))
            .await
    {
        short_pause();
        next_page_button.wait_until().clickable().await?;
        println!("Next page button is clickable");
        next_page_button.click().await?;
        println!("Next page button clicked");
        super::short_pause();
    }
    Ok(())
}

async fn get_all_entry(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    driver.find_all(By::Css(
        "#all-content > search-app > search-base-search-holder > search-results > div > div > div.job-results-container > search-job-cards > mat-accordion > mat-expansion-panel"
        )).await
}

/// Check if the next page button is clickable
/// If it is not clickable or the button is not found, return false
async fn next_page_button_exists_or_clickable(driver: &WebDriver) -> Result<bool, WebDriverError> {
    if let Ok(next_page_button) = driver
        .find(By::XPath(
            "/html/body/div[2]/search-app/search-base-search-holder/search-results/div/search-job-paginator/mat-paginator/div/div/div[2]/button[2]",
        ))
        .await {
    println!("Next page button found");
    return Ok(next_page_button.is_clickable().await?)
    }
    println!("Next page button not found");
    Ok(false)
}

async fn record_entry_to_csv(
    driver: &WebDriver,
    wtr: &mut csv::Writer<std::fs::File>,
) -> Result<(), Box<dyn Error>> {
    let all_entry = get_all_entry(driver).await?;
    println!("There are {} entries", all_entry.len());
    for id in 0..all_entry.len() {
        if let Ok(mut tmp) = job_entry_from_elementid(driver, id).await {
            tmp.location = "CN".to_string();
            tmp.company_name = "Amd".to_string();
            wtr.serialize(tmp)?;
        }
    }

    Ok(())
}

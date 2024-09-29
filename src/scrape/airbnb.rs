//! This module provide scrape function
//! which scrapes interns jobs posted on
//! "https://careers.airbnb.com/positions/?_departments=early-career-program-intern"
//! For all jobs, see  (they are not scraped)
//! "https://careers.airbnb.com/positions/"

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
    let save_to = format!("{}airbnb.csv", crate::CONFIG.lock().unwrap().raw_data_dir);
    println!(
        "{}, saving to {save_to}",
        "Scraping Airbnb job".yellow().bold(),
    );
    scrape_site(
        driver,
        "https://careers.airbnb.com/positions/?_departments=early-career-program-intern",
        save_to.as_str(),
    )
    .await
}

// for serializing to csv
async fn job_entry_from_element(element: &WebElement) -> Result<JobEntry, WebDriverError> {
    let title = element.find(By::Css("h3 > a")).await?.text().await?;
    let url = element
        .find(By::Css("h3 > a"))
        .await?
        .attr("href")
        .await?
        .unwrap_or_default();
    println!("Title: {}; href: {}", title, url);
    let mut location = "".to_string();
    let location_elements = element.find_all(By::XPath("div[2]/span")).await?;
    for location_element in location_elements {
        let tmp = location_element.text().await?;
        location = format!("{}, {}", location, tmp);
    }
    // strip leading comma and space 
    location = location[2..].to_string();
    println!("Location: {}", location);
    Ok(JobEntry {
        job_title: title,
        apply_link: url,
        location: location,
        ..Default::default() // default defined in main
    })
}

async fn scrape_site(driver: &WebDriver, url: &str, save_to: &str) -> Result<(), Box<dyn Error>> {
    let url_tmp = Url::parse(url)?;
    driver.goto(url_tmp).await?;
    println!("{} at {}", "Scraping Airbnb job".yellow().bold(), url);
    super::short_pause();

    // No pop-up this site
    // click_popup(driver).await?;
    // super::short_pause();

    let mut wtr = csv::Writer::from_path(save_to)?;
    println!("Writing to {}", save_to);
    let all_entry = get_all_entry(driver).await?;
    println!("Found {} entries", all_entry.len());
    for entry in all_entry {
        let mut tmp = job_entry_from_element(&entry).await?;
        tmp.company_name = "Airbnb".to_string();
        wtr.serialize(tmp)?;
    }
    Ok(())
}

/// this site does not seem to have a popup menu
// async fn click_popup(driver: &WebDriver) -> Result<(), Box<dyn Error>> {
//     if let Ok(popup_menu_ok_button) = driver
//         .find(By::XPath("/html/body/dialog[1]/div[2]/button[2]"))
//         .await
//     {
//         popup_menu_ok_button.wait_until().clickable().await?;
//         popup_menu_ok_button.click().await?;
//         return Ok(());
//     }
//     println!("No popup menu found; continuing...");
//     return Ok(());
// }

async fn get_all_entry(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
    driver
        .find_all(By::XPath("/html/body/div/div[2]/div[2]/div"))
        .await
}

use std::process::{self, Command};

use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use url::Url;

use colored::Colorize;
use std::sync::Mutex;

// TODO: Remove unsafe code.
// https://stackoverflow.com/questions/78268471/rust-kill-stdprocessschild-after-finishing-executing
static mut CHILD_PROCESS_ID: i32 = 0;

// static DRIVER: Mutex<Option<WebDriver>> = Mutex::new(None);

extern "C" fn kill_child() {
    unsafe {
        let pid: libc::pid_t = CHILD_PROCESS_ID;
        libc::kill(pid, libc::SIGKILL);
    }
}

fn run_chrome_driver() {
    if std::path::Path::new("chromedriver").exists() {
        println!("ChromeDriver already exists!");
        println!("Running ChromeDriver ... ");
        let child = Command::new("./chromedriver")
            .spawn()
            .expect("Failed To Run Chromedriver");
        unsafe {
            let id: i32 = child
                .id()
                .try_into()
                .expect("Failure converting u32 to i32 in web_driver::run_chrome_driver()");
            CHILD_PROCESS_ID = id;
            libc::atexit(kill_child);
        }
    } else {
        panic!(
            "{}\n{}\n{}",
            "Chrome Driver does not exist!",
            "Download The Chrome Driver!".red().bold(),
            "Please Download the Chrome Driver with the same version as your browser. See readme.md"
            );
    }
}

/// Initialize and run the driver
pub async fn initialize_driver(auto_run_driver: bool) -> Result<WebDriver, WebDriverError> {
    if auto_run_driver {
        run_chrome_driver();
    }

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                "Failed To Connects to Chrome Driver at port 9515"
                    .bold()
                    .red()
            )
        });
    driver.maximize_window().await?;
    Ok(driver)
}

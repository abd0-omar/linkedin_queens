// #[cfg(target_os = "windows")]
use std::env;
use thirtyfour::error::WebDriverErrorInfo;
use thirtyfour::prelude::*;

pub async fn start_browser() -> WebDriverResult<(WebDriver, String)> {
    // Define the base user data directory and the specific profile directory
    // These paths must exactly match how Edge stores them on your system.
    // Use `edge://version/` in your Edge browser to confirm.
    // I'm running on windows btw, I think I'm using the btw word here wrong
    // #[cfg(target_os = "windows")]
    let home_dir = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let user_data_dir = format!("{}/edge_automation_profile", home_dir);
    // let user_data_dir = "C:\\Users\\abdel\\AppData\\Local\\Microsoft\\Edge\\User Data";

    // let profile_directory = "Default"; // Use the name of your specific profile folder

    // Initialize Edge capabilities with minimal startup time
    let mut caps = DesiredCapabilities::edge();

    // Add arguments to load the specific user profile
    // The --user-data-dir argument points to the parent folder containing profiles
    caps.add_arg(&format!("--user-data-dir={}", user_data_dir))?;
    // The --profile-directory argument specifies which profile within user-data-dir to use
    // caps.add_arg(&format!("--profile-directory={}", profile_directory))?;
    // Add performance optimizations
    // caps.add_arg("--disable-gpu")?;
    // caps.add_arg("--no-sandbox")?;
    // caps.add_arg("--disable-dev-shm-usage")?;
    // caps.add_arg("--disable-extensions")?;
    // caps.add_arg("--disable-notifications")?;

    // IMPORTANT: Ensure msedgedriver is running and listening on this port.
    // If you're running it manually, verify the port it outputs.
    let driver = WebDriver::new("http://localhost:9586", caps).await?;

    // Set a shorter implicit wait timeout for element location, by
    // default it's 0
    // driver
    //     .set_implicit_wait_timeout(Duration::from_secs(1))
    //     .await?;

    // Navigate to LinkedIn Queens game
    driver
        .goto("https://www.linkedin.com/games/view/queens/desktop")
        .await?;

    // Wait for and click the start game button with aggressive polling
    let mut attempts = 0;
    let max_attempts = i32::MAX; // More attempts but shorter intervals
    let start_button = loop {
        match driver.find(By::Id("launch-footer-start-button")).await {
            Ok(button) => break button,
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    // If we've exhausted attempts, return the error
                    return Err(e);
                }
                // A possible very short sleep between attempts
                // tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    };
    start_button.click().await?;

    // Try to find and click the dismiss button immediately
    if let Ok(dismiss_button) = driver
        .find(By::Css("button[data-test-modal-close-btn]"))
        .await
    {
        println!("Found and clicking dismiss button.");
        dismiss_button.click().await?;
    } else {
        println!("Dismiss button not found or not needed.");
    }

    // Wait for the game board with aggressive polling
    let mut attempts = 0;
    let max_attempts = i32::MAX;
    let board = loop {
        match driver.find(By::Id("queens-grid")).await {
            Ok(board_element) => {
                // Check if the board has content immediately
                if let Ok(cells) = board_element
                    .find_all(By::Css("div.queens-cell-with-border"))
                    .await
                {
                    if !cells.is_empty() {
                        println!("Game board and cells found. Board is loaded.");
                        break board_element;
                    }
                }
                attempts += 1;
                if attempts >= max_attempts {
                    // If we found the board but no cells, return what we have
                    println!(
                        "Warning: Game board found, but no cells after max attempts. Returning current board state."
                    );
                    break board_element;
                }
                // A possible very short sleep between content checks
                // tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    // If we can't find the board at all after max attempts, return error
                    return Err(WebDriverError::NoSuchElement(WebDriverErrorInfo::new(
                        format!(
                            "Could not find queens-grid element after {} attempts: {}",
                            max_attempts, e
                        ),
                    )));
                }
                // A possible very short sleep between attempts
                // tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    };

    // Get the outer HTML of the board element
    let board_html = board.outer_html().await?;
    println!("Successfully captured board HTML.");

    // Return the driver and the HTML
    // The driver is not quit here, so the browser stays open for inspection.
    Ok((driver, board_html))
}

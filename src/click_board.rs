use thirtyfour::By;
use thirtyfour::prelude::*;

pub async fn click_solution_squares(
    driver: &WebDriver,
    solution_indices: &[usize],
) -> WebDriverResult<()> {
    // Wait for the grid to be present
    let grid = driver.find(By::Id("queens-grid")).await?;

    // For each solution index, find and click the corresponding cell
    for &idx in solution_indices {
        // Find the cell using data-cell-idx attribute
        let cell = grid
            .find(By::Css(format!("[data-cell-idx=\"{}\"]", idx).as_str()))
            .await?;

        // Double click the cell
        cell.click().await?;
        cell.click().await?;

        // Small delay between clicks to avoid overwhelming the browser
        // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(59)).await;

    Ok(())
}

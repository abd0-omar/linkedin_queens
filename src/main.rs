use linkedin_queens::{click_board::click_solution_squares, parse_board, queens, start_browser};

#[tokio::main]
async fn main() {
    // Get the board HTML from the browser
    match start_browser().await {
        Ok((driver, board_html)) => {
            // Parse the HTML into a board
            let board = parse_board(&board_html);

            // Try to solve the queens puzzle
            match queens(&board) {
                Ok(result) => {
                    println!("Successfully solved the puzzle! {:?}", result);
                    // Click the solution squares
                    if let Err(e) = click_solution_squares(&driver, &result).await {
                        println!("Error clicking solution squares: {}", e);
                    }
                }
                Err(e) => println!("Error solving puzzle: {}", e),
            }
        }
        Err(e) => println!("Error getting board from browser: {}", e),
    }
}

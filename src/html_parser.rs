use scraper::{Html, Selector};

use crate::game_logic::CellColor;

pub fn parse_board(html_content: &str) -> Vec<Vec<CellColor>> {
    let document = Html::parse_document(&html_content);

    // Get board dimensions from the grid style
    let grid_selector = Selector::parse("div#queens-grid").unwrap();
    let grid = document.select(&grid_selector).next().unwrap();
    let style = grid.value().attr("style").unwrap();
    let rows = style
        .split("--rows: ")
        .nth(1)
        .unwrap()
        .split(";")
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let cols = style
        .split("--cols: ")
        .nth(1)
        .unwrap()
        .split(";")
        .next()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    // it's always a n*n board, so rows == cols
    println!("Board dimensions: {}x{}", rows, cols);

    let cell_selector = Selector::parse("div.queens-cell-with-border").unwrap();
    let mut board = vec![vec![CellColor::Lavender; cols]; rows];

    for cell in document.select(&cell_selector) {
        let label = cell.value().attr("aria-label").unwrap();
        let row = label
            .split("row ")
            .nth(1)
            .and_then(|s| s.split(",").next())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap()
            - 1; // Convert to 0-based index

        let col = label
            .split("column ")
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap()
            - 1; // Convert to 0-based index

        println!("label: {}, row: {}, col: {}", label, row, col);

        let cell_color = CellColor::from_aria_label(label).unwrap();

        board[row][col] = cell_color;
    }

    for row in &board {
        println!("{:?}", row);
    }

    board
}

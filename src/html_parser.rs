use scraper::{Html, Selector};

use crate::game_logic::CellColor;

pub fn parse_board(html_content: &str) -> Vec<Vec<CellColor>> {
    println!("Parsing HTML content of length: {}", html_content.len());
    let document = Html::parse_document(html_content);

    // Get board dimensions from the grid style
    let grid_selector = Selector::parse("div#queens-grid").expect("Failed to parse grid selector");
    let grid = document
        .select(&grid_selector)
        .next()
        .expect("Could not find queens-grid element");

    let style = grid
        .value()
        .attr("style")
        .expect("Grid element has no style attribute");
    println!("Found grid style: {}", style);

    let rows = style
        .split("--rows: ")
        .nth(1)
        .expect("Could not find --rows in style")
        .split(";")
        .next()
        .expect("Could not parse rows value")
        .trim()
        .parse::<usize>()
        .expect("Could not parse rows as number");

    let cols = style
        .split("--cols: ")
        .nth(1)
        .expect("Could not find --cols in style")
        .split(";")
        .next()
        .expect("Could not parse cols value")
        .trim()
        .parse::<usize>()
        .expect("Could not parse cols as number");

    // it's always a n*n board, so rows == cols
    println!("Board dimensions: {}x{}", rows, cols);

    let cell_selector =
        Selector::parse("div.queens-cell-with-border").expect("Failed to parse cell selector");
    let mut board = vec![vec![CellColor::Lavender; cols]; rows];
    let mut cells_found = 0;

    for cell in document.select(&cell_selector) {
        let label = cell
            .value()
            .attr("aria-label")
            .expect("Cell has no aria-label");
        let row = label
            .split("row ")
            .nth(1)
            .and_then(|s| s.split(",").next())
            .and_then(|s| s.parse::<usize>().ok())
            .expect("Could not parse row number")
            - 1; // Convert to 0-based index

        let col = label
            .split("column ")
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .expect("Could not parse column number")
            - 1; // Convert to 0-based index

        println!("Found cell: label='{}', row={}, col={}", label, row, col);

        let cell_color = CellColor::from_aria_label(label).expect("Could not parse cell color");
        board[row][col] = cell_color;
        cells_found += 1;
    }

    println!(
        "Found {} cells out of {} total cells",
        cells_found,
        rows * cols
    );
    println!("Final board:");
    for row in &board {
        println!("{:?}", row);
    }

    board
}

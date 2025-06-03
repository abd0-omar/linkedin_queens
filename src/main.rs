use linkedin_queens::{game_logic::queens, html_parser::parse_board};

fn main() {
    let html_content = include_str!("../html_board2.html");
    let board = parse_board(&html_content);
    queens(&board).unwrap();
}

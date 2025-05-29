use std::collections::HashSet;

fn main() {
    let mut board = vec![
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Orange,
            CellColor::Orange,
            CellColor::Orange,
            CellColor::Blue,
            CellColor::Blue,
            CellColor::Blue,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Orange,
            CellColor::Blue,
            CellColor::Blue,
            CellColor::Blue,
            CellColor::Blue,
        ],
        vec![
            CellColor::Purple,
            CellColor::Green,
            CellColor::Green,
            CellColor::Green,
            CellColor::Blue,
            CellColor::Gray,
            CellColor::Blue,
            CellColor::Blue,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Green,
            CellColor::Red,
            CellColor::Blue,
            CellColor::Gray,
            CellColor::Gray,
            CellColor::Yellow,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Red,
            CellColor::Red,
            CellColor::Blue,
            CellColor::Gray,
            CellColor::Yellow,
            CellColor::Yellow,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Red,
            CellColor::DarkGray,
            CellColor::DarkGray,
            CellColor::DarkGray,
            CellColor::Yellow,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::DarkGray,
            CellColor::Purple,
            CellColor::Purple,
        ],
        vec![
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
            CellColor::Purple,
        ],
    ];
    let mut status: Vec<Vec<Option<Status>>> = vec![vec![None; 8]; 8];
    // the game uses backtrack to solve the game, it doesn't rely on marking
    // cells with (can not be a queen) by applying the rules, it's just a
    // backtracking brute-force solution
    let mut colors: HashSet<CellColor> = HashSet::with_capacity(8);
    if backtrack(&mut board, &mut status, 0, 8, &mut colors) {
        println!("Solution found")
    } else {
        println!("No solution found")
    }
    // dbg!(&status);
    // dbg!(&board);
}

fn backtrack(
    board: &mut Vec<Vec<CellColor>>,
    status: &mut Vec<Vec<Option<Status>>>,
    row: usize,
    n: usize,
    colors: &mut HashSet<CellColor>,
) -> bool {
    if row == n {
        return true;
    }

    for col in 0..n {
        if is_valid(board, status, row, col, n, colors) {
            // change state
            status[row][col] = Some(Status::Queen);
            colors.insert(board[row][col]);
            // backtrack
            if backtrack(board, status, row + 1, n, colors) {
                return true;
            }
            // undo change
            status[row][col] = None;
            colors.remove(&board[row][col]);
        }
    }
    false
}

fn is_valid(
    board: &mut Vec<Vec<CellColor>>,
    status: &mut Vec<Vec<Option<Status>>>,
    row: usize,
    col: usize,
    n: usize,
    colors: &HashSet<CellColor>,
) -> bool {
    // no need to check row because we put only one queen in each row
    // check col, before the row, col
    // check one distance diagonal, before the row, col
    // can't be in the same grid color (dfs)
    // every color grid must have a queen

    // check previous columns
    for r in 0..row {
        if status[r][col].is_some() {
            return false;
        }
    }

    // check previous one distance diagonals
    for (nr, nc) in [
        (row.wrapping_sub(1), col.wrapping_sub(1)), // up-left
        (row.wrapping_sub(1), col + 1),             // up-right
    ] {
        if (0..n).contains(&nr) && (0..n).contains(&nc) {
            if status[nr][nc].is_some() {
                return false;
            }
        }
    }

    // let mut visited = vec![vec![false; n]; n];
    // if dfs_found_queen_in_same_color_grid(board, status, board[row][col], row, col, &mut visited) {
    //     return false;
    // }
    if colors.contains(&board[row][col]) {
        return false;
    }
    true
}

// redundant, replaced it with hashset
#[allow(unused)]
fn dfs_found_queen_in_same_color_grid(
    board: &[Vec<CellColor>],
    status: &[Vec<Option<Status>>],
    cur_color: CellColor,
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if i == board.len() {
        return false;
    }
    if board[i][j] != cur_color {
        return false;
    }
    if visited[i][j] {
        return true;
    }
    if status[i][j].is_some() {
        return true;
    }

    visited[i][j] = true;

    // 8 directions
    for (ni, nj) in [
        (i, j.wrapping_sub(1)),
        (i.wrapping_sub(1), j),
        (i + 1, j),
        (i, j + 1),
        (i.wrapping_sub(1), j.wrapping_sub(1)),
        (i + 1, j + 1),
        (i.wrapping_sub(1), j + 1),
        (i + 1, j.wrapping_sub(1)),
    ] {
        if (0..board.len()).contains(&ni) && (0..board[0].len()).contains(&nj) && !visited[ni][nj] {
            if dfs_found_queen_in_same_color_grid(board, status, cur_color, ni, nj, visited) {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellColor {
    Green,
    Blue,
    Red,
    Yellow,
    Purple,
    Orange,
    Gray,
    DarkGray,
}

#[derive(Debug, Clone, Copy)]
enum Status {
    Queen,
    // won't use the below variant
    // CantBeAQueen,
}

// N-Queens LeetCode problem
// pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
//     let n = n as usize;
//     let mut board = vec![vec!['.'; n]; n];
//     let mut result = Vec::new();
//     backtrack(&mut board, 0, n, &mut result);
//     dbg!(board);
//     result
//         .into_iter()
//         .map(|b| {
//             b.into_iter()
//                 .map(|row| row.into_iter().collect::<String>())
//                 .collect()
//         })
//         .collect()
// }

// fn backtrack(board: &mut Vec<Vec<char>>, row: usize, n: usize, result: &mut Vec<Vec<Vec<char>>>) {
//     // row will have one queen for a valid solution
//     if row == n {
//         dbg!(&board);
//         result.push(board.clone());
//     }
//     for col in 0..n {
//         if is_valid(board, row, col) {
//             // change state
//             board[row][col] = 'Q';
//             // backtrack
//             backtrack(board, row + 1, n, result);
//             // undo change
//             board[row][col] = '.';
//         }
//     }
// }

// fn is_valid(board: &[Vec<char>], row: usize, col: usize) -> bool {
//     let n = board.len();
//     // Check column
//     for r in 0..row {
//         if board[r][col] == 'Q' {
//             return false;
//         }
//     }
//     // Check left diagonal (going up-left)
//     let mut r = row as i32 - 1;
//     let mut c = col as i32 - 1;
//     while r >= 0 && c >= 0 {
//         if board[r as usize][c as usize] == 'Q' {
//             return false;
//         }
//         r -= 1;
//         c -= 1;
//     }
//     // Check right diagonal (going up-right)
//     let mut r = row as i32 - 1;
//     let mut c = col as i32 + 1;
//     while r >= 0 && c < n as i32 {
//         if board[r as usize][c as usize] == 'Q' {
//             return false;
//         }
//         r -= 1;
//         c += 1;
//     }
//     true
// }

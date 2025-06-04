use std::collections::HashSet;

pub fn queens(board: &Vec<Vec<CellColor>>) -> Result<(), String> {
    let mut status: Vec<Vec<Option<Status>>> = vec![vec![None; board.len()]; board.len()];
    // the game uses backtrack to solve the game, it doesn't rely on marking
    // cells with (can not be a queen) by applying the rules, it's just a
    // backtracking brute-force solution
    let mut colors: HashSet<CellColor> = HashSet::with_capacity(board.len());
    if !backtrack(&board, &mut status, 0, board.len(), &mut colors) {
        return Err("No solution found".to_string());
    }
    for row in &status {
        println!("{:?}", row);
    }
    Ok(())
}

fn backtrack(
    board: &Vec<Vec<CellColor>>,
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
    board: &Vec<Vec<CellColor>>,
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
pub enum CellColor {
    PeachOrange,
    SoftBlue,
    PastelGreen,
    LightGray,
    VibrantCoral,
    LimeYellow,
    Lavender,
    WarmBeige,
    DarkGray,
    Pink,
}

impl CellColor {
    pub fn from_aria_label(label: &str) -> Option<Self> {
        if label.contains("Peach Orange") {
            Some(CellColor::PeachOrange)
        } else if label.contains("Soft Blue") {
            Some(CellColor::SoftBlue)
        } else if label.contains("Pastel Green") {
            Some(CellColor::PastelGreen)
        } else if label.contains("Light Gray") {
            Some(CellColor::LightGray)
        } else if label.contains("Vibrant Coral") {
            Some(CellColor::VibrantCoral)
        } else if label.contains("Lime Yellow") {
            Some(CellColor::LimeYellow)
        } else if label.contains("Lavender") {
            Some(CellColor::Lavender)
        } else if label.contains("Warm Beige") {
            Some(CellColor::WarmBeige)
        } else if label.contains("Dark Gray") {
            Some(CellColor::DarkGray)
        } else if label.contains("Pink") {
            Some(CellColor::Pink)
        } else {
            None
        }
    }
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

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SudokuRequest {
    grid: [[u8; 9]; 9],
}

#[derive(Serialize, Deserialize)]
struct SudokuResponse {
    solution: [[u8; 9]; 9],
    solved: bool,
}

fn solve_sudoku(grid: &mut [[u8; 9]; 9]) -> bool {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut subgrids = [0u16; 9];

    for row in 0..9 {
        for col in 0..9 {
            let num = grid[row][col];
            if num != 0 {
                let mask = 1 << num;
                rows[row] |= mask;
                cols[col] |= mask;
                subgrids[(row / 3) * 3 + (col / 3)] |= mask;
            }
        }
    }

    solve(grid, &mut rows, &mut cols, &mut subgrids)
}

fn solve(
    grid: &mut [[u8; 9]; 9],
    rows: &mut [u16; 9],
    cols: &mut [u16; 9],
    subgrids: &mut [u16; 9],
) -> bool {
    if let Some((row, col)) = find_empty_cell(grid) {
        for num in 1..=9 {
            let mask = 1 << num;
            if (rows[row] & mask) == 0
                && (cols[col] & mask) == 0
                && (subgrids[(row / 3) * 3 + (col / 3)] & mask) == 0
            {
                grid[row][col] = num;
                rows[row] |= mask;
                cols[col] |= mask;
                subgrids[(row / 3) * 3 + (col / 3)] |= mask;

                if solve(grid, rows, cols, subgrids) {
                    return true;
                }

                grid[row][col] = 0;
                rows[row] &= !mask;
                cols[col] &= !mask;
                subgrids[(row / 3) * 3 + (col / 3)] &= !mask;
            }
        }
        false
    } else {
        true
    }
}

fn find_empty_cell(grid: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    let mut min_possibilities = 10;
    let mut target_cell = None;

    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] == 0 {
                let mut count = 0;
                for num in 1..=9 {
                    if is_valid(grid, row, col, num) {
                        count += 1;
                    }
                }
                if count < min_possibilities {
                    min_possibilities = count;
                    target_cell = Some((row, col));
                }
            }
        }
    }
    target_cell
}

fn is_valid(grid: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for i in 0..9 {
        if grid[row][i] == num || grid[i][col] == num {
            return false;
        }
    }
    let start_row = row / 3 * 3;
    let start_col = col / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if grid[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }
    true
}

fn is_valid_grid(grid: &[[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] > 9 {
                return false;
            }
        }
    }

    for row in 0..9 {
        for col in 0..9 {
            let num = grid[row][col];
            if num != 0 {
                let mut grid_copy = *grid;
                grid_copy[row][col] = 0;
                if !is_valid(&grid_copy, row, col, num) {
                    return false;
                }
            }
        }
    }
    true
}

async fn sudoku_endpoint(req: web::Json<SudokuRequest>) -> impl Responder {
    let mut grid = req.grid;
    if !is_valid_grid(&grid) {
        return HttpResponse::BadRequest().json("Invalid Sudoku grid");
    }

    let solved = solve_sudoku(&mut grid);
    if solved {
        HttpResponse::Ok().json(SudokuResponse {
            solution: grid,
            solved: true,
        })
    } else {
        HttpResponse::BadRequest().json("Sudoku puzzle is unsolvable")
    }
}

async fn validate_sudoku_endpoint(req: web::Json<SudokuRequest>) -> impl Responder {
    let grid = req.grid;
    if is_valid_grid(&grid) {
        HttpResponse::Ok().json("Valid Sudoku grid")
    } else {
        HttpResponse::NotFound().json("Invalid Sudoku grid")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/solve", web::post().to(sudoku_endpoint))
            .route("/validate", web::put().to(validate_sudoku_endpoint))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

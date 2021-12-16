pub type Board = Vec<Vec<u32>>;

fn search(number: u32, board: &Board) -> Option<(usize, usize)> {
    // assuming this number appear only once
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == number {
                return Some((row, col));
            }
        }
    }
    None
}

fn remove(number: u32, v: &mut Vec<u32>) -> Option<u32> {
    if let Some(idx) = v.iter().position(|x| *x == number) {
        Some(v.swap_remove(idx))
    } else {
        None
    }
}

pub fn part1(numbers: &[u32], boards: &[Board]) -> u32 {

    // Store the remaining number in row/number for fast query
    // board_idx -> rows -> number
    let mut rows: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut cols: Vec<Vec<Vec<u32>>> = Vec::new();

    for board in boards {
        // Create place for this board
        let mut board_rows: Vec<Vec<u32>> = vec![Vec::new(); 5];
        let mut board_cols: Vec<Vec<u32>> = vec![Vec::new(); 5];
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                board_rows[row].push(board[row][col]);
                board_cols[col].push(board[row][col]);
            }
        }
        rows.push(board_rows);
        cols.push(board_cols);
    }


    for number in numbers {
        for (board_idx, board) in boards.iter().enumerate() {
            println!("number {}" , number);
            if let Some((row, col)) = search(*number, board) {
                println!("Removing {} - {}", row, col);
                remove(*number, &mut rows[board_idx][row]);
                remove(*number, &mut cols[board_idx][col]);
                if rows[board_idx][row].is_empty() || cols[board_idx][col].is_empty() {
                    // sum all unmmarked numbers
                    let unmarked_numbers_sum: u32 = rows[board_idx]
                    .iter()
                    .map(|row| row.iter().sum::<u32>())
                    .sum();
                    println!("board: {}, number: {}, unmarked number sums: {}", board_idx, number, unmarked_numbers_sum);
                    return unmarked_numbers_sum  * number;
                } else {
                    println!("board: {}, row length {}, column length {}", board_idx, rows[board_idx].len(), cols[board_idx].len());
                }
            }

        }
    }

    0
}

//TODO: refactor
pub fn part2(numbers: &[u32], boards: &[Board]) -> u32 {
    // Store the remaining number in row/number for fast query
    // board_idx -> rows -> number
    let mut rows: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut cols: Vec<Vec<Vec<u32>>> = Vec::new();

    for board in boards {
        // Create place for this board
        let mut board_rows: Vec<Vec<u32>> = vec![Vec::new(); 5];
        let mut board_cols: Vec<Vec<u32>> = vec![Vec::new(); 5];
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                board_rows[row].push(board[row][col]);
                board_cols[col].push(board[row][col]);
            }
        }
        rows.push(board_rows);
        cols.push(board_cols);
    }

    let mut winners: Vec<usize> = Vec::new();
    let mut last_number: u32 = 1;

    for number in numbers {
        last_number = *number;
        for (board_idx, board) in boards.iter().enumerate() {
            if winners.contains(&board_idx) {
                continue
            }
            if let Some((row, col)) = search(*number, board) {
                println!("Removing number {} from board {} at {} - {}", *number, board_idx, row, col);
                remove(*number, &mut rows[board_idx][row]);
                remove(*number, &mut cols[board_idx][col]);
                if rows[board_idx][row].is_empty() || cols[board_idx][col].is_empty() {
                    println!("Board {} won", board_idx);
                    winners.push(board_idx)
                }
            }
        }
        if winners.len() == boards.len() {
            break;
        }
    }

    let last_winner = winners.last().unwrap();
    let unmarked_sum: u32 = rows[*last_winner]
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum();
    println!("Last winner: {}, unmarked sum: {}, last number: {}", last_winner, unmarked_sum, last_winner);
    unmarked_sum * last_number
}

pub fn search_row(board: [[i32;9];9], row:usize, num:i32) -> bool{
    for i in 0..9{
        for j in 0..9{
            if board[row][j] == num{
                return false;
            }
        }
    }
    return true;
}   

pub fn search_col(board: [[i32;9];9], col:usize, num:i32) -> bool{
    for i in 0..9{
        for j in 0..9{
            if board[i][col] == num{
                return false;
            }
        }
    }
    return true;
}

pub fn search_matrix(board: [[i32;9];9], row:i32, col:i32, num:i32) -> bool{
    let r = row - row % 3;
    let c = col - col % 3;

    for i in 0..3{
        for j in 0..3{
            if board[(r+i) as usize][(c+j) as usize] == num{
                return false;
            }
        }
    }
    return true;
}
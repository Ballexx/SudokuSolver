use crate::modules::finder::{search_col, search_row, search_matrix};

pub fn show_board(board: [[i32;9];9]){
    for i in board{
        for j in i{
            print!("{} ", j);
        }
        print!("\r\n");
    }
}

fn find_empty(board: [[i32;9];9]) -> (i32, i32){
    for i in 0..9{
        for j in 0..9{
            if board[i][j] == 0{
                return (i as i32, j as i32);
            }
        }
    }
    return (-1,-1);
}

pub fn calculate(board: &mut[[i32;9];9]) -> bool{
    let (i, j) = find_empty(*board);

    if i==-1 && j == -1{
        return true
    }

    for num in 1..10{
        if search_row(*board, i as usize, num) && search_col(*board, j as usize, num) && search_matrix(*board, i, j, num){
            board[i as usize][j as usize] = num;   

            if calculate(board){
                return true;
            }

            board[i as usize][j as usize] = 0;

        }
    }
    return false;
}   
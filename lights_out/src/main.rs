use std::io;
use rand::Rng;

fn main() {
    println!("Lights out start!");
    
    let mut board: [bool; ROW_COLUMN_NUM.pow(2)] = [false; ROW_COLUMN_NUM.pow(2)];
    init_random_board(&mut board, &ROW_COLUMN_NUM);
    const ROW_COLUMN_NUM: usize = 3;
    let mut loop_finish_flag = true;

    view_board(&mut board, &ROW_COLUMN_NUM);

    while loop_finish_flag {
        println!("0 ~ 9までの数字を入力してください");
        let mut cell_idx = String::new();
    
        io::stdin().read_line(&mut cell_idx).expect("標準入力を受け付けませんでした。");
        let cell_idx: usize = cell_idx.trim().parse().expect("0 ~ 9までの正の整数を入力してください");
    
        if cell_idx < 0 || cell_idx >  board.len() - 1{
            println!("範囲外の数字が入力されました");
        }
    
        change_board(&mut board, &cell_idx, &ROW_COLUMN_NUM);
    
        finish_judge(&board, &mut loop_finish_flag);
        view_board(&mut board, &ROW_COLUMN_NUM);
    }

}

fn change_board(board: &mut [bool], cell_idx: &usize, ROW_COLUMN_NUM: &usize) {
    let upper : i8 = *cell_idx as i8 - *ROW_COLUMN_NUM as i8;
    let lower : i8 = *cell_idx as i8 + *ROW_COLUMN_NUM as i8;
    let left : i8 = *cell_idx as i8 - 1;
    let right: i8 = *cell_idx as i8 + 1;

    let board_check: [i8; 5] = [upper, lower, left, right, *cell_idx as i8];
    
    for check_cell_idx in board_check {
        if check_cell_idx < 0 || check_cell_idx > ROW_COLUMN_NUM.pow(2) as i8 - 1 {
            continue
        }

        board[check_cell_idx as usize] = !board[check_cell_idx as usize];
    }
}

fn view_board(board: &[bool], ROW_COLUMN_NUM: &usize) {
    for row in 0..*ROW_COLUMN_NUM as i8 {
        print!("|");
        for column in 0..*ROW_COLUMN_NUM as i8 {
            let cell_idx = *ROW_COLUMN_NUM as i8 * row + column;
            let cell = board[cell_idx as usize];
            print!(" {} |",board[cell_idx as usize] );
        } 
        println!("");
    }
}

fn finish_judge(board : &[bool], loop_finish_flag: &mut bool) {
    for element in board {
        if *element {
            return;
        }
    }
    *loop_finish_flag = false;
}

fn init_random_board(board: &mut[bool],  ROW_COLUMN_NUM: &usize) {
    let board_element_num : i8 = ROW_COLUMN_NUM.pow(2) as i8 - 1;
    let random_cell_num = rand::thread_rng().gen_range(0..board_element_num);

    println!("{}", random_cell_num);
    for i in  1..=random_cell_num {
        loop {
            let random_cell_idx : usize = rand::thread_rng().gen_range(0..board_element_num) as usize;
            if !board[random_cell_idx] {
                board[random_cell_idx] = true;
                break;
            }
        }
    }
}
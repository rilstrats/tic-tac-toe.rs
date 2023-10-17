use std::io;

fn main() {
    let mut current_player: char = 'X';
    let mut board: Vec<char> = ('0'..='9').collect();
    // for i in '0'..='9' {
    //     board.push(i)
    // }

    for _ in 0..9 {
        let mut guess: String = String::default();
        display_board(&board);
        println!("Player {}'s turn", current_player);
        print!("Choose your square: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        switch_player(current_player);
    }
}

fn display_board(board: &Vec<char>) {
    println!("{}|{}|{}", board[1], board[2], board[3]);
    println!("-+-+-");
    println!("{}|{}|{}", board[4], board[5], board[6]);
    println!("-+-+-");
    println!("{}|{}|{}", board[7], board[8], board[9]);
}

fn switch_player(mut current_player: char) {
    if current_player == 'X' {
        current_player = 'O'
    } else {
        current_player = 'X'
    }
}

use std::{fmt, io};

fn main() {
    run_game()
}

const WIN_CASES: [(usize, usize, usize); 8] = [
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    (0, 4, 8),
    (2, 4, 6),
];

fn run_game() {
    let mut board = Board::new();
    let mut current_player = Player::new();

    for _ in 0..9 {
        current_player.take_turn(&mut board);
        match board.win_check() {
            true => {
                println!("\n-----------------------------\n");
                println!("{} wins!", current_player);
                std::process::exit(0);
            }
            false => (),
        }
        current_player.switch();
    }

    println!("\nCat. No one wins.")
}

#[derive(Debug)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X => write!(f, "Player X"),
            Self::O => write!(f, "Player O"),
        }
    }
}

impl Player {
    fn new() -> Self {
        Self::X
    }
    fn switch(&mut self) {
        match self {
            Self::X => *self = Self::O,
            Self::O => *self = Self::X,
        }
    }
    fn take_turn(&self, board: &mut Board) {
        println!("\n-----------------------------\n");
        board.display();
        loop {
            println!("{}'s turn, type a square number to claim: ", self);
            let mut input_string: String = Default::default();
            match io::stdin().read_line(&mut input_string) {
                Ok(_) => (),
                Err(_) => {
                    println!("\nNo input detected!");
                    continue;
                }
            }
            match input_string.trim().parse::<usize>() {
                Err(_) | Ok(0) => {
                    println!("\nPlease type a number from 1 to 9!");
                    continue;
                }
                Ok(n) => match board.claim_square(n, &self) {
                    Ok(_) => break,
                    Err(e) => {
                        println!("\n{}", e);
                        continue;
                    }
                },
            }
        }
    }
}

#[derive(Debug)]
enum BoardError {
    SquareClaimed(usize),
    InvalidSquare(usize),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::SquareClaimed(n) => write!(f, "Square {} has already been claimed!", n),
            Self::InvalidSquare(n) => {
                write!(f, "Square {} doesn't exist, pick a square from 1 to 9!", n)
            }
        }
    }
}

#[derive(Debug)]
struct Square {
    num: usize,
    state: SquareState,
}

#[derive(Debug)]
enum SquareState {
    X,
    O,
    Free,
}

impl From<Player> for SquareState {
    fn from(player: Player) -> Self {
        match player {
            Player::X => SquareState::X,
            Player::O => SquareState::O,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            SquareState::X => write!(f, "X"),
            SquareState::O => write!(f, "O"),
            SquareState::Free => write!(f, "{}", self.num),
        }
    }
}

impl From<usize> for Square {
    fn from(n: usize) -> Self {
        Self {
            num: n,
            state: SquareState::Free,
        }
    }
}

impl Square {
    // fn new(square_num: u8) -> Self {
    //     Self::Free(square_num)
    // }
    fn claim(&mut self, current_player: &&Player) -> Result<(), BoardError> {
        match (&self.state, current_player) {
            (SquareState::Free, Player::X) => Ok(self.state = SquareState::X),
            (SquareState::Free, Player::O) => Ok(self.state = SquareState::O),
            (SquareState::X | SquareState::O, _) => Err(BoardError::SquareClaimed(self.num)),
        }
    }
}

#[derive(Debug)]
struct Board {
    squares: [Square; 9],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}|{}|{}",
            self.squares[0], self.squares[1], self.squares[2]
        )?;
        writeln!(f, "-+-+-")?;
        writeln!(
            f,
            "{}|{}|{}",
            self.squares[3], self.squares[4], self.squares[5]
        )?;
        writeln!(f, "-+-+-")?;
        writeln!(
            f,
            "{}|{}|{}",
            self.squares[6], self.squares[7], self.squares[8]
        )
    }
}

impl Board {
    fn new() -> Self {
        Self {
            squares: (1..=9)
                .map(|x| Square::from(x))
                .collect::<Vec<Square>>()
                .try_into()
                .expect("Unable to initialize Board.squares"),
        }
    }

    fn claim_square(
        &mut self,
        square_num: usize,
        current_player: &&Player,
    ) -> Result<(), BoardError> {
        self.squares
            .get_mut(square_num - 1)
            .ok_or(BoardError::InvalidSquare(square_num))?
            .claim(current_player)
    }

    fn display(&self) {
        println!("{}", self)
    }

    fn win_check(&self) -> bool {
        for win_case in WIN_CASES {
            match (
                &self.squares[win_case.0].state,
                &self.squares[win_case.1].state,
                &self.squares[win_case.2].state,
            ) {
                (SquareState::X, SquareState::X, SquareState::X)
                | (SquareState::O, SquareState::O, SquareState::O) => return true,
                (_, _, _) => continue,
            }
        }
        false
    }
}

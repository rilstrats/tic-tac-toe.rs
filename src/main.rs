use std::fmt;

fn main() {
    let board = Board::new();
    println!("{}", board);
}

#[derive(Clone)]
enum Square {
    X,
    O,
    Free,
}

impl Default for Square {
    fn default() -> Self {
        Self::Free
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
            Square::Free => write!(f, " "),
        }?;
        Ok(())
    }
}

struct Board {
    squares: Vec<Square>,
}

impl Board {
    fn new() -> Self {
        Self {
            squares: vec![Square::default(); 9],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            " {} | {} | {} ",
            self.squares[0], self.squares[1], self.squares[2]
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {} | {} | {} ",
            self.squares[3], self.squares[4], self.squares[5]
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {} | {} | {} ",
            self.squares[6], self.squares[7], self.squares[8]
        )?;
        Ok(())
    }
}

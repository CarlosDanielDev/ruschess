use crate::{Color, Piece};

pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            squares: std::array::from_fn(|_| None),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Piece> {
        self.squares[index].as_ref()
    }

    pub fn set(&mut self, index: usize, piece: Piece) {
        self.squares[index] = Some(piece)
    }

    pub fn count(&self, color: Color) -> usize {
        let mut total: usize = 0;

        for square in self.squares.iter() {
            match square {
                Some(p) if p.color == color => total += 1,
                _ => {}
            }
        }

        total
    }
}

pub mod board;
pub mod exercises;

pub use board::Board;

#[derive(Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[derive(Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub fn describe(p: &Piece) -> String {
    format!("{:?} {:?}", p.kind, p.color)
}

pub fn demo() {
    let knight = Piece {
        kind: PieceKind::Knight,
        color: Color::White,
    };

    let a = describe(&knight);
    let b = describe(&knight);

    println!("{a} {b}");
}


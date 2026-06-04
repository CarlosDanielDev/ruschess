use domain::{
    PieceKind,
    moves::{CastleSide, describe_move},
};

fn main() {
    domain::exercises::demo_promote();
    println!("---------------");
    domain::exercises::demo_flip_color();
    println!("---------------");
    domain::exercises::demo_flip_sequential();
    println!("---------------");
    let mut board = domain::Board::empty();
    let pawn = domain::Piece {
        kind: domain::PieceKind::Pawn,
        color: domain::Color::White,
    };

    let knight = domain::Piece {
        kind: domain::PieceKind::Knight,
        color: domain::Color::White,
    };

    let rook = domain::Piece {
        kind: domain::PieceKind::Rook,
        color: domain::Color::Black,
    };

    board.set(0, pawn);
    board.set(1, knight);
    board.set(7, rook);
    let total_white = board.count(domain::Color::White);
    let total_black = board.count(domain::Color::Black);

    println!("square 0: {:?}", board.get(0));
    println!("square 1: {:?}", board.get(1));
    println!("square 7: {:?}", board.get(7));
    println!("total {:?}: {:?}", domain::Color::White, total_white);
    println!("total {:?}: {:?}", domain::Color::Black, total_black);
    println!("---------------");

    let square = domain::Square::new(1).unwrap();
    let square_to = domain::Square::new(2).unwrap();

    let quiet = domain::Move::Quiet {
        from: square,
        to: square_to,
    };

    let capture = domain::Move::Capture {
        from: square,
        to: square_to,
    };

    let castle = domain::Move::Castle {
        side: CastleSide::KingSide,
    };

    let en_passant = domain::Move::EnPassant {
        from: square,
        to: square_to,
    };

    let promotion = domain::Move::Promotion {
        from: square_to,
        to: square,
        to_kind: PieceKind::Rook,
        is_capture: true,
    };

    println!("quiet: {:?}", describe_move(&quiet));
    println!("capture: {:?}", describe_move(&capture));
    println!("castle: {:?}", describe_move(&castle));
    println!("en passant: {:?}", describe_move(&en_passant));
    println!("promotion: {:?}", describe_move(&promotion));
}

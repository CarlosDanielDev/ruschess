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
}

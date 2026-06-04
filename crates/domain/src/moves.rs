use crate::{PieceKind, Square};

#[derive(Debug)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

#[derive(Debug)]
pub enum Move {
    Quiet {
        from: Square,
        to: Square,
    },
    Capture {
        from: Square,
        to: Square,
    },
    Castle {
        side: CastleSide,
    },
    EnPassant {
        from: Square,
        to: Square,
    },
    Promotion {
        from: Square,
        to: Square,
        to_kind: PieceKind,
        is_capture: bool,
    },
}

pub fn describe_move(m: &Move) -> String {
    match m {
        Move::Quiet { from, to } => format!("{:?} -> {:?}", from, to),
        Move::Capture { from, to } => format!("{:?} x {:?}", from, to),
        Move::EnPassant { from, to } => format!("{:?} ep {:?}", from, to),
        Move::Castle { side } => format!("castle {:?}", side),
        Move::Promotion {
            from,
            to,
            to_kind,
            is_capture,
        } => {
            let sep = if *is_capture { "x" } else { "->" };
            format!("{:?} {} -> {:?} = {:?}", from, sep, to, to_kind)
        }
    }
}

//  pub fn describe_move(m: &Move) -> String {
//      match m {
//          Move::Quiet   { from, to } => format!("{:?} -> {:?}", from, to),
//          Move::Capture { from, to } => format!("{:?} x {:?}", from, to),
//      }
//  }
//
//  rustc:
//
//  error[E0004]: non-exhaustive patterns: `&Move::Castle { .. }`, `&Move::EnPassant { .. }` and `&Move::Promotion { .. }` not covered
//    --> crates/domain/src/moves.rs:4:11
//     |
//   4 |     match m {
//     |           ^ patterns `&Move::Castle { .. }`, `&Move::EnPassant { .. }` and `&Move::Promotion { .. }` not covered
//     |
//  note: `Move` defined here
//    --> crates/domain/src/moves.rs:10:5
//     |
//   7 | pub enum Move {
//     |          ----
//  ...
//  10 |     Castle    { side: CastleSide },
//     |     ^^^^^^ not covered
//     = help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown

//  pub fn is_promotion(m: &Move) -> bool {
//      match m {
//          Move::Promotion { .. } => true,
//          Move::Quiet { .. } => false,
//          Move::Capture { .. } => false,
//          Move::EnPassant { .. } => false,
//          Move::Castle { .. } => false,
//      }
//  }

// pub fn is_promotion(m: Move) -> bool {
//     if let Move::Promotion { .. } = m {
//         true
//     } else {
//         false
//     }
// }

pub fn is_promotion(m: &Move) -> bool {
    matches!(m, Move::Promotion { .. })
}

pub fn describe_index(i: u8) -> String {
    let Some(sq) = Square::new(i) else {
        return String::from("invalid");
    };

    format!("file: {:?}, rank: {:?}", sq.file(), sq.rank())
}

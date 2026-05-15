use crate::{Color, Piece, PieceKind};
// Exercise 1 — promote consumes the pawn and returns a queen.
//
// FAIL VERSION (what we tried first):
//
//   pub fn demo_promote() {
//       let pawn = Piece { kind: PieceKind::Pawn, color: Color::White };
//       let promoted = promote(pawn);
//       println!("promoted: {:?}", promoted);
//       println!("pawn: {:?}", pawn); // ← fails
//   }
//
// rustc error:
//
//   error[E0382]: borrow of moved value: `pawn`
//     --> crates/domain/src/exercises.rs:17:28
//      |
//   10 |     let pawn = Piece { kind: PieceKind::Pawn, color: Color::White };
//      |         ---- move occurs because `pawn` has type `Piece`,
//      |              which does not implement the `Copy` trait
//   ...
//   14 |     let promoted = promote(pawn);
//      |                            ---- value moved here
//   ...
//   17 |     println!("pawn: {:?}", pawn);
//      |                            ^^^^ value borrowed here after move
//
// Fix: don't try to use `pawn` after handing ownership to `promote`.
// Promotion *consumes* the pawn — that's the whole point.

pub fn promote(p: Piece) -> Piece {
    Piece {
        kind: PieceKind::Queen,
        color: p.color,
    }
}

pub fn demo_promote() {
    let pawn = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };
    println!("before promotion: {:?}", pawn.kind);

    let promoted = promote(pawn);

    println!("after promotion: {:?}", promoted.kind);
}

// Concept 2 — &mut piece exclusivity.
//
// FAIL VERSION:
//
//   pub fn demo_borrow_fail() {
//       let mut piece = Piece { kind: PieceKind::Pawn, color: Color::White };
//       let r1 = &mut piece;
//       let r2 = &mut piece;
//       println!("{:?} {:?}", r1, r2);
//   }
//
// rustc:
//
//   error[E0499]: cannot borrow `piece` as mutable more than once at a time
//     --> crates/domain/src/exercises.rs:55:14
//      |
//   54 |     let r1 = &mut piece;
//      |              ---------- first mutable borrow occurs here
//   55 |     let r2 = &mut piece;
//      |              ^^^^^^^^^^ second mutable borrow occurs here
//   ...
//   57 |     println!("{:?} {:?}", r1, r2);
//      |                           -- first borrow later used here
//
// Fix: stagger the borrows so they don't overlap (NLL drops r1 at its last use).

pub fn demo_borrow_sequential() {
    let mut piece = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };

    let r1 = &mut piece;
    println!("{:?}", r1);
    let r2 = &mut piece;
    println!("{:?}", r2);
}

pub fn demo_borrow_ok() {
    let piece = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };

    let r1 = &piece;
    let r2 = &piece;
    let r3 = &piece;

    println!("{:?} {:?} {:?}", r1, r2, r3);
}

pub fn flip_color(p: &mut Piece) {
    p.color = match p.color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

pub fn demo_flip_color() {
    let mut piece = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };
    println!("Before: {:?}", piece.color);

    flip_color(&mut piece);

    println!("After flip 1: {:?}", piece.color);

    flip_color(&mut piece);

    println!("After flip 2: {:?}", piece.color);
}

pub fn demo_flip_sequential() {
    let mut piece = Piece {
        kind: PieceKind::Pawn,
        color: Color::White,
    };

    let r1 = &mut piece;
    flip_color(r1);
    let r2 = &mut piece;
    flip_color(r2);
    println!("Staggered: {:?}", piece.color);
}

// Concept 2 in flip context — two &mut piece overlap → E0499.
//
// FAIL VERSION:
//
//   pub fn demo_flip_double_borrow_fail() {
//       let mut piece = Piece { kind: PieceKind::Pawn, color: Color::White };
//       let r1 = &mut piece;
//       let r2 = &mut piece;
//       flip_color(r1);
//       flip_color(r2);
//   }
//
// rustc:
//
//   error[E0499]: cannot borrow `piece` as mutable more than once at a time
//     --> crates/domain/src/exercises.rs:131:14
//      |
//   130 |     let r1 = &mut piece;
//      |              ---------- first mutable borrow occurs here
//   131 |     let r2 = &mut piece;
//      |              ^^^^^^^^^^ second mutable borrow occurs here
//   132 |
//   133 |     flip_color(r1);
//      |                -- first borrow later used here
//
// Fix: stagger the &mut so they never overlap.

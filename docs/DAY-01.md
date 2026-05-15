# Day 1 — Ownership, Borrowing, Workspace

**Time budget:** 2 hours
**Prerequisite:** rustc + cargo installed, `rueschess` repo cloned, hello world compiling.

## Goal

By end of session you can:

- Explain ownership, `&T`, and `&mut T` in TS-analogy form without notes.
- Read and fix a "value borrowed after move" rustc error on sight.
- Read and fix a "cannot borrow as mutable more than once" error on sight.
- Navigate a Cargo workspace and add a module to a library crate.

## Acceptance criteria

- `crates/domain/src/lib.rs` defines `Piece`, `PieceKind`, `Color`.
- `crates/domain/src/board.rs` exists with a `Board` struct holding 64
  `Option<Piece>` squares and `empty`, `get`, `set`, `count` methods.
- `crates/domain/src/exercises.rs` contains the three exercises below,
  each compiling, each preceded by a comment showing the original
  fail-to-compile version and the exact rustc error it produced.
- `cargo check --workspace` passes.
- `docs/SPIKE.md` has a "Day 1" section.
- `docs/CONTINUATION-PROMPT.md` is updated.

## Pre-flight (15 min)

1. Verify workspace from repo root:

```bash
   cargo check --workspace
```

If the root `Cargo.toml` doesn't declare members yet, add:

```toml
   [workspace]
   members = ["crates/*"]
   resolver = "2"
```

2. Create the three crates if missing:

```bash
   cargo new --lib crates/domain
   cargo new --lib crates/engine
   cargo new --bin crates/tui
```

3. Open `crates/domain/src/lib.rs` in neovim. Delete the default test.

## Concept 1 — Ownership (30 min)

**TS analogy.** In TS, passing an object to a function hands over a reference;
you keep using the original after. In Rust, passing a non-`Copy` value _moves_
it — the original binding becomes invalid. As if TS had a `consumed: true`
flag the type system enforced.

**Why Rust needs this.** No GC. Exactly one owner is responsible for freeing
memory. Multiple owners would mean use-after-free or double-free. Rust trades
GC for compile-time ownership tracking.

**Fail-to-compile snippet.** In `crates/domain/src/lib.rs`:

```rust
#[derive(Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

#[derive(Debug)]
pub enum PieceKind { Pawn, Knight, Bishop, Rook, Queen, King }

#[derive(Debug)]
pub enum Color { White, Black }

fn describe(p: Piece) -> String {
    format!("{:?} {:?}", p.color, p.kind)
}

pub fn demo() {
    let knight = Piece { kind: PieceKind::Knight, color: Color::White };
    let a = describe(knight);
    let b = describe(knight); // ← will fail
    println!("{a} {b}");
}
```

**Run** `cargo check -p domain`. Expect:

```
error[E0382]: use of moved value: `knight`
  --> ...
   |     let a = describe(knight);
   |                      ----- value moved here
   |     let b = describe(knight);
   |                      ^^^^^ value used here after move
```

**Three valid fixes:**

1. Make `describe` borrow: `fn describe(p: &Piece) -> String`. This is the
   right one — _reading shouldn't consume_.
2. `#[derive(Clone)]` and `describe(knight.clone())` — wasteful here.
3. `#[derive(Copy, Clone)]` — works only because all fields are `Copy`. Tempting
   trap; we'll learn when `Copy` is appropriate later. Skip for now.

**Where the TS analogy breaks.** TS never errors on the second call. Rust
forces you to declare intent up front: am I reading, mutating, or consuming?

**Docs:** Rust Book §4.1 — https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

**Exercise 1** (in `crates/domain/src/exercises.rs`):
Write `pub fn promote(p: Piece) -> Piece` that takes a pawn and returns a
queen of the same color. Then write `pub fn demo_promote()` that creates a
pawn, promotes it, and prints both — **without** using `Clone`. Hint: the
original pawn shouldn't exist after promotion. That's literally what
promotion means.

## Concept 2 — Shared and exclusive borrows (40 min)

**TS analogy.** `&T` is `readonly T`. `&mut T` is a mutable reference, with
a twist: at any moment, EITHER many `&T` OR exactly one `&mut T`. Never both.
TS has nothing like this rule. It's how Rust prevents data races at compile
time (and even single-threaded aliasing bugs).

**Fail-to-compile snippet:**

```rust
pub fn demo_borrow() {
    let mut piece = Piece { kind: PieceKind::Pawn, color: Color::White };
    let r1 = &mut piece;
    let r2 = &mut piece; // ← fail
    println!("{:?} {:?}", r1, r2);
}
```

**Expected error:**

```
error[E0499]: cannot borrow `piece` as mutable more than once at a time
```

**Why.** If `r1` and `r2` could mutate `piece` at once, you'd have a race
(or, single-threaded, surprise aliasing). Rust says: one writer, or many
readers, never mixed.

**The fix.** Use `r1` first, let its scope end, then take `r2`. Or use one
mutable reference total. Or use shared `&` if you don't need to mutate.

**Companion — shared borrows are fine, many at once:**

```rust
let piece = Piece { kind: PieceKind::Pawn, color: Color::White };
let r1 = &piece;
let r2 = &piece;
let r3 = &piece;
println!("{:?} {:?} {:?}", r1, r2, r3); // OK
```

**Docs:** Rust Book §4.2 — https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

**Exercise 2.** In `exercises.rs`, write `pub fn flip_color(p: &mut Piece)`
that toggles White↔Black. Then write `pub fn demo_flip()` that creates a
piece, flips twice, prints between each flip. Then **intentionally** break
it by trying to hold two `&mut` at once. Paste the exact rustc error as a
comment. Then fix it.

## Concept 3 — Modules and visibility (35 min)

**TS analogy.** A Rust module is a file or a `mod {}` block. `pub` is
`export`. `use` is `import`. A library crate's `src/lib.rs` is its `index.ts`.

**Steps:**

1. Create `crates/domain/src/board.rs`:

```rust
   use crate::{Color, Piece};

   pub struct Board {
       squares: [Option<Piece>; 64],
   }

   impl Board {
       pub fn empty() -> Self {
           Self { squares: std::array::from_fn(|_| None) }
       }

       pub fn get(&self, index: usize) -> Option<&Piece> {
           self.squares[index].as_ref()
       }

       pub fn set(&mut self, index: usize, piece: Piece) {
           self.squares[index] = Some(piece);
       }
   }
```

2. In `crates/domain/src/lib.rs`, declare and re-export the module:

```rust
   pub mod board;
   pub use board::Board;
```

3. Run `cargo check -p domain`. Fix visibility errors as they appear.
   Expected sticking point: if you forget `pub` on `Piece` fields, `board.rs`
   can read `Piece` but can't construct one with field syntax. Read the
   error, add `pub`, retry.

**The "no `Clone`" gotcha.** `board.set(0, piece)` moves `piece` into the
board. After that, `piece.kind` won't compile. This is correct — the piece
_lives on the board now_. We don't paper over it with `Clone` yet.

**Docs:**

- Modules: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Visibility: https://doc.rust-lang.org/reference/visibility-and-privacy.html

**Exercise 3.** Add `impl Board { pub fn count(&self, color: Color) -> usize }`
that returns how many pieces of the given color are on the board. You'll
iterate `self.squares.iter()` and pattern-match on `Option<&Piece>`. Reach
for `match`; don't fake it with `if let` chains. (Side quest: `Color` needs
`PartialEq` to be compared with `==`. Derive it.)

## Wrap-up (20 min)

1. **Append to `docs/SPIKE.md`:**

```markdown
## Day 1 — Ownership, Borrowing, Workspace

### Concepts learned

- Ownership: non-Copy values move on pass; original binding is invalid.
- `&T` vs `&mut T`: shared XOR exclusive, enforced at compile time.
- Module system: `mod`, `pub`, `use`, `pub use` re-exports.

### TS↔Rust analogies that landed

- `&T` ≈ `readonly T` param.
- `pub mod board; pub use board::Board;` ≈ barrel re-export from index.ts.
- "Move" has no clean TS analogy — closest: "imagine passing consumed the object."

### Gotchas

- Forgot `pub` on struct fields; board.rs couldn't construct `Piece`.
- Two `&mut` in same scope; had to learn that scopes end at last use (NLL).
- `Color` needed `PartialEq` derived to use `==`.

### Docs consulted

- <paste the links you actually opened>
```

2. **Update `docs/CONTINUATION-PROMPT.md`** with:
   - Current branch + last commit hash.
   - One-line status: "Day 1 complete. Ready for Day 2: domain modeling with
     enums and pattern matching."
   - Open questions to ask tomorrow.

3. **Commit:**

```bash
   git add -A
   git commit -m "day-01: ownership, borrowing, workspace bootstrap"
```

## If you finish early

Read Rust Book §4.3 (slices). It's the bridge to Day 2. Don't code on it yet.

## If you run out of time

Skip Exercise 3 and add it to `docs/CONTINUATION-PROMPT.md` as carry-over.
Concept 2 (borrows) is the must-have for Day 2 to make sense.

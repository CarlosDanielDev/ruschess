# SPIKE — rueschess: TUI chess in Rust

## Spike question

Can a TS dev with Rust basics ship a working TUI chess game with a Stockfish
opponent in 14 hours of focused study (7 days × 2h), and end the week with
the architectural seams needed to swap Stockfish for the Lucidmate API later?

## Scope

- Cargo workspace: `domain`, `engine`, `tui`.
- Move generation via `shakmaty`. No hand-rolled chess rules.
- TUI via `ratatui` + `crossterm`. Keyboard input only.
- Engine via Stockfish over UCI (subprocess + stdio).
- Local hot-seat and human-vs-engine play. No clocks, no PGN export.

## Non-goals (this week)

- Network play, matchmaking, accounts.
- Move animations, sound, themes.
- Opening books, endgame tablebases.
- Pure-Rust engine (e.g. `pleco`) — out of scope; UCI is the seam.
- Lucidmate API integration — planned for week 2, not this spike.

## Decisions log

| Decision           | Choice                               | Why                                                                                                                                     | Date  |
| ------------------ | ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- | ----- |
| Move generation    | `shakmaty`                           | Most maintained chess crate, FEN/PGN built-in, lets us focus on Rust idioms instead of reinventing rules.                               | Day 0 |
| TUI stack          | `ratatui` + `crossterm`              | Industry default, immediate-mode is closer to React mental model than `cursive`.                                                        | Day 0 |
| Engine             | Stockfish via UCI subprocess         | "Popular" requirement met; teaches `std::process::Command` + stdio piping, which is the same shape Lucidmate's engine bridge will need. | Day 0 |
| Workspace layout   | 3 crates (`domain`, `engine`, `tui`) | Forces module-boundary thinking early — the thing TS lets you be sloppy about.                                                          | Day 0 |
| Async runtime      | None this week; sync only            | UCI subprocess I/O is fine over blocking threads. Adding `tokio` here is premature.                                                     | Day 0 |
| Engine abstraction | `trait ChessEngine` introduced Day 7 | Keeps `tui` decoupled from Stockfish so Lucidmate becomes another `impl`.                                                               | Day 0 |

## Open questions

- [ ] How does `shakmaty` represent a `Move` internally? Will our `domain::Move`
      become a thin wrapper or do we re-export theirs?
- [ ] Best ergonomic pattern for the game loop in `ratatui` — `App` struct with
      `update`/`view` or pure event-match? (Decide Day 5.)
- [ ] Stockfish path: hardcode `stockfish`, env var `STOCKFISH_BIN`, or config

## Day 1 — Ownership, Borrowing, Workspace (2026-05-14 → 2026-05-15)

### Concepts learned

- Ownership: non-`Copy` values move on pass; original binding becomes invalid.
- `&T` vs `&mut T`: shared XOR exclusive, enforced at compile time.
- Non-lexical lifetimes (NLL): borrows end at last use, not at end of scope.
- Match scrutinee rule: scrutinee is a *place* (read-only) when no arm binds by value; becomes a move only when an arm binds (e.g. `Some(x) => ...` with `x` taking inner ownership).
- Module system: `mod foo;` declares a file is part of the build; `pub` exports; `use` imports; `pub use` is the barrel re-export.
- Cargo workspaces: `[workspace] members = ["crates/*"]`; folder path and package `name` are independent — renaming a folder doesn't rename the package.

### TS↔Rust analogies that landed

- `&T` ≈ `readonly T` reference, enforced by the type system (not vibes).
- `pub mod board; pub use board::Board;` ≈ barrel re-export from `index.ts`.
- "Move" has no clean TS analogue — closest: imagine TS errored on use-after-pass.
- `Option<T>` = `T | undefined`, but **exhaustive** — every reader must handle both.
- `match` ≈ `switch` that the compiler forces to be total.
- `if cond { value }` with no `else` evaluates to `()`, not `value` — surprised me twice.

### Gotchas

- `cargo new --bin crates/ui` named the package `ui` after the folder. Renamed folder to `tui` via `mv` but had to **also** edit `crates/tui/Cargo.toml` `name = "tui"` — they're independent.
- `cargo check workspace` fails: `workspace` parses as a bad positional. Need `--workspace` (double dash).
- `==` requires `#[derive(PartialEq)]`. Not built in.
- `#[derive(Copy, Clone)]` hides moves — tempting trap, deliberately skipped on `Piece` / `Color`.
- `if cond { piece }` with no `else` → "expected (), found Piece". Every `if` arm must yield the same type, and missing `else` defaults to `()`.
- `[None; 64]` doesn't compile when `T` isn't `Copy`. Used `std::array::from_fn(|_| None)`.
- `match p.color` where `p: &mut Piece` did **not** fail with a move error because the arms (`Color::White =>`, `Color::Black =>`) bind nothing. I expected a move; learned the place vs value rule.

### Docs consulted

- Rust Book §4.1 Ownership — https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Rust Book §4.2 References & Borrowing — https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Rust Book ch07 Modules — https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Rust Reference — Place vs value expressions — https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions
- Rust Reference — Match expressions — https://doc.rust-lang.org/reference/expressions/match-expr.html
- NLL announcement — https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018/#non-lexical-lifetimes
- Cargo workspaces — https://doc.rust-lang.org/cargo/reference/workspaces.html
- `std::array::from_fn` — https://doc.rust-lang.org/std/array/fn.from_fn.html
- `Option::as_ref` — https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref

### Deliverables

- Cargo workspace with `domain`, `engine`, `tui` crates.
- `crates/domain/src/lib.rs`: `Piece`, `PieceKind`, `Color` (derive `Debug`; `Color` also derives `PartialEq`).
- `crates/domain/src/board.rs`: `Board` (`[Option<Piece>; 64]`) with `empty / get / set / count`.
- `crates/domain/src/exercises.rs`: `promote`, `flip_color`, `demo_*` functions. Each broken version preserved as a `//` comment block above its working sibling, with the exact rustc error quoted.
- `crates/tui/src/main.rs`: wires every demo and runs them.
- `cargo check --workspace` passes. `cargo run -p tui` prints expected output for promote / flip / board demos.

## Day 2 — Enums with data, pattern matching, Option (2026-06-02 → 2026-06-04)

### Concepts learned

- Enums-with-data = TS discriminated unions, but the variant *is* the tag (no hand-maintained `kind` string field). `Move::Quiet { from, to }`, `Move::Promotion { from, to, to_kind, is_capture }`, etc.
- `Copy` decided by **what a value owns**, not its size or whether the thing it represents changes over time. `Copy` allowed only when a bitwise (byte-for-byte) duplicate is harmless — i.e. the type owns no resource needing cleanup. `Copy` and `Drop` are mutually exclusive.
- `Square` newtype `struct Square(u8)` is `Copy` (owns one `u8`, behaves like a TS `number`). `Piece` was deliberately left non-`Copy` in Day 1 to feel moves — it *could* be `Copy`; that was a teaching choice, not a hard limit.
- Four "pick a variant" tools, each for a different job:
  - `match` — handle every variant differently (`describe_move`).
  - `matches!(x, Pat)` — yes/no on one variant; expands to the `if let .. else` bool form (`is_promotion`).
  - `if let Pat = x { .. }` — peek one variant, optionally grab its fields.
  - `let Pat = x else { diverge };` — unwrap-or-bail guard clause; `else` must `return`/`panic!`/`break` (`describe_index`).
- Binding through a `&` reference: in `match m` where `m: &Move`, field bindings come out as references (`is_capture: &bool`). Deref with `*` to get the value; cheap because `bool` is `Copy` (deref-copies the small value out, no move).
- `{ .. }` in a pattern ignores remaining fields — adding `is_capture` to `Promotion` did NOT break `is_promotion`'s `Move::Promotion { .. }`.

### TS↔Rust analogies that landed

- `Copy` = behaves like a TS `number` (`const b = a` leaves `a` usable); not-`Copy` = behaves like an object that gets *consumed* on pass (the Day 1 move error).
- `matches!` = `m.kind === "promotion"`.
- `let else` = the early-return guard clause: `const sq = Square.new(i); if (sq === undefined) return;` then `sq` is known-valid below.
- enum-with-data = `type Move = { kind:'quiet', ... } | { kind:'castle', ... }`, minus the manual `kind` field.

### Gotchas

- First `is_promotion` took `m: Move` (owned) — a predicate should borrow (`&Move`), not consume the caller's value.
- First `is_promotion` used a full 5-arm `match` for a yes/no — reserved the exhaustive tool for the wrong job; `matches!` is one line.
- `if *is_capture` needs the `*`: the binding is `&bool`, `if` wants `bool`.
- Comment bug in `square.rs`: file column comment said `h(8)`; `% 8` yields `0..=7`, so it's `h(7)`. (Code was correct, comment wrong — fixed.)

### Domain-modeling decision

- Capturing promotion (e.g. b7xa8=Q) was unrepresentable: `Capture` had no `to_kind`, `Promotion` had no capture info. Chose **Option A** — add `is_capture: bool` to `Promotion` — over folding into shakmaty-style optional fields. Rationale: explicit, own model, no premature complexity (CLAUDE.md: no premature abstraction).

### Docs consulted

- Rust Book §6 Enums & pattern matching — https://doc.rust-lang.org/book/ch06-00-enums.html
- Rust Book §18 Patterns — https://doc.rust-lang.org/book/ch18-00-patterns.html
- `std::marker::Copy` (see "When can my type be Copy?" / Copy vs Drop) — https://doc.rust-lang.org/std/marker/trait.Copy.html
- `let`-else statements — https://doc.rust-lang.org/reference/statements.html#let-statements
- `matches!` macro — https://doc.rust-lang.org/std/macro.matches.html

### Deliverables

- `crates/domain/src/square.rs`: `Square(u8)` newtype, `Copy`; `new -> Option<Square>` (guards `>= 64`), `index`, `rank`, `file`.
- `crates/domain/src/moves.rs`: `CastleSide`, `Move` (5 variants incl. `Promotion { .., is_capture }`), `describe_move` (exhaustive), `is_promotion` (`matches!`), `describe_index` (`let else`). Broken versions preserved as comments with exact rustc errors per convention.
- `crates/domain/src/lib.rs`: declares + re-exports `Square`, `Move`.
- `cargo check --workspace` clean.

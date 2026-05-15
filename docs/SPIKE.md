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

The 7-day plan

Day 1 — Ownership, borrowing, modules. Set up the workspace, refresh the borrow checker with chess-flavored exercises (move a Piece struct around, get the compiler to yell at you, learn why). TS analogy lens: &T vs &mut T vs owned T mapped to "readonly reference / mutable reference / value transfer."

Day 2 — Domain modeling with enums and pattern matching. Build Color, PieceKind, Square, Move as enums/structs. This is where Rust's type system clicks vs TS — exhaustive match, no null, Option<T> instead of T | undefined. Compare discriminated unions to Rust enums with data.

Day 3 — Integrate shakmaty, parse FEN, render ASCII board. First real external crate. Learn Result<T, E> and the ? operator (TS: imagine if every function that could throw returned Either<E, T> and ? was syntactic sugar for if (isErr) return err). Output: a CLI binary that takes a FEN string and prints the board.

Day 4 — Game loop, parse moves from stdin, play human-vs-human. Loops, String vs &str (the eternal Rust footgun — TS has no equivalent, this is where you slow down). Build a working terminal chess game with no AI yet.

Day 5 — ratatui intro and board widget. Learn the immediate-mode render loop, App state struct, KeyEvent handling via crossterm. Replace your ASCII print with a real TUI: bordered board, status line, move history panel.

Day 6 — Stockfish UCI integration. Spawn Stockfish, wire up stdin/stdout pipes, send position fen ... and go depth 10, parse bestmove responses. This is the "talking to a subprocess" pattern you'll reuse forever.

Day 7 — Traits, polish, tests, refactor. Introduce a ChessEngine trait so Stockfish is swappable (this is your bridge to Lucidmate later — your API becomes another impl ChessEngine). Write a couple of #[test] functions. Refactor the spike code into something you'd actually ship. Output: SPIKE.md with everything you learned and NEXT.md for the Lucidmate integration plan.

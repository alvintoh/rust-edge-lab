# Rust course — from zero to an edge telemetry gateway

- **Learner:** new to Rust; experienced software engineer (TypeScript, Go-familiar,
  event-driven integrations).
- **Goal:** by **2027-03-31**, write a Rust program that reads a solar charge controller
  over Modbus and reports what it sees, then move on to embedded Rust on an ESP32.
- **Pace:** about 5–7 hours a week from 2026-11-17; the holiday weeks are buffer.
- **Toolchain:** Rust 1.100.0 (rustup, with the `rust-analyzer` component) in WSL and on
  Windows, and on the Mac in Ghostty; Zed in WSL mode on Windows, native on the Mac.
- **Hardware:** none needed before Module 9 (Jan–Feb purchase).

## How this course works

1. **You write every line.** The tutor gives the shape (signatures, a flow diagram,
   leading questions) and reviews what you wrote; the bodies are yours.
2. **Each module runs one loop:** learn (the Book section is named when its Rustlings group
   starts), drill (Rustlings), RETRIEVE (a closed-book quiz of about 5 questions, no
   cheatsheet), BUILD (one exercise crate in this workspace, from a blank file), REVIEW.
3. **Every exercise has an oracle**: a hand-computed fixture, a simulator with known
   values, or an existing program to diff against. The borrow checker accepting your code
   proves it is memory-safe, not that it is right.
4. **Small on purpose:** each crate stays under about 500 lines.
5. **One module is reviewed before the next starts**, and each ends in a commit.
   **Build before drilling ahead:** a module's build (the crate and its oracle) comes before
   the next module's Rustlings group, and a group finished early waits for its module's turn.
   *(Owner's instruction 2026-11-17: drilling ran ahead of building, so the build is the
   checkpoint that sets the pace.)*
6. **Study by testing and spacing, not rereading.** Each session opens with 3 questions from
   EARLIER modules, taken from the cheatsheet headings; each build opens with the 5-question
   quiz. *(Established: Dunlosky et al. 2013 rated practice testing and distributed practice
   highest utility and rereading low; the question counts are a judgment call.)*
   **The tutor asks each quiz question ONE at a time** (the code in the message text, the
   answer by letter in chat) and **grades each answer with real compiler output before asking
   the next**, then gives a running score. *(Owner's instructions 2026-11-17 and 2026-11-19:
   one question per prompt is easier to answer, and checking after every question is easier to
   learn from than a grade at the end; a judgment call.)* **The tutor shuffles the options so
   the correct letter cannot be guessed from a pattern, and checks the letters afterwards: a
   run of the same correct letter is a defect in the quiz, not a skill.** *(Owner's catch
   2026-11-19: the first four retake answers were all B; a judgment call.)* **Weak spots are
   re-asked first.** An idea answered wrongly in a quiz goes into
   the "Weak spots" table and is asked at the start of every session until it is answered
   correctly in two different sessions. *(Judgment call; it applies the spacing rule to what
   you actually missed.)*
7. **Oracle first:** write the `assert_eq!` before the function body. *(Judgment call.)*
   **The tutor scaffolds each build file:** the signatures, structs and `impl` blocks with
   `todo!()` bodies, a comment on each item carrying its decision and leading question, and
   the hand-computed cases listed as comments in `main`. The `assert_eq!` checks and every
   body stay yours. *(Owner's instruction 2026-11-19: the shape is easier to follow in the
   file than in chat; a judgment call.)*
   **Before you write each item, the tutor explains it in full, in this order:** what the
   real-world thing is (a register, a scale), why the item exists, a run example with its
   real output pasted, what the item does read straight off that output, what is already in
   your file and what it does, then numbered steps that name the file and line. No item is
   handed over as a signature alone. *(Owner's instruction 2026-11-19, after
   `combine_registers` was confusing as a signature plus a formula and clear once explained
   this way; a judgment call.)*
8. **Hint ladder:** question, hint, shape, reveal. The Record table notes the highest rung
   used per module; a build finished at "shape" or below is a pass, and one that needed
   "reveal" is repeated smaller. *(Judgment call.)*

## General

**Mode:** tutor

### Modules

| # | Weeks | Read + drill | Build (crate) | Oracle |
|---|---|---|---|---|
| 1 | 1–2 | Book ch. 1–3; Rustlings intro, variables, functions, if, primitive_types | `m1-units`: unit conversions | known values, checked with `assert_eq!` |
| 2 | 3–4 | ch. 4–5; Rustlings move_semantics, structs | `m2-registers`: 32-bit values from two registers; a scaled `Reading` | hand-computed cases |
| 3 | 5–6 | ch. 6–8; Rustlings enums, strings, modules, hashmaps, quiz2, options (vecs done early) | `m3-signals`: Modbus function codes as an enum; a deadband filter; latest value per register | fixtures |
| 4 | 7–8 | ch. 9–10; Rustlings error_handling, generics, traits, quiz3, lifetimes | `m4-regmap`: parse a register map from CSV | a known map, including bad rows |
| 5 | 9–10 | ch. 11, 13; Rustlings tests, iterators | `m5-summary`: min/max/avg/last/count over a window | hand-computed |
| 6 | 11–12 | ch. 12 (the CLI project); Rustlings conversions, macros, clippy | `m6-contrast`: a Rust port of a WCAG contrast checker | the original Python tool, run on the same public SVGs |
| 7 | 13–14 | ch. 15–16; Rustlings smart_pointers, threads; the Tokio tutorial | `m7-poller`: fake sensor every second → channel → deadband → summary | a deterministic fake sensor |
| 8 | 15–16 | `tokio-modbus` docs | `m8-modbus-tcp`: read and decode registers from a Modbus TCP simulator | the simulator's configured values |
| 9 | 17+ | The Embedded Rust Book (QEMU chapter first) | switch Module 8 to RTU over RS-485 on a real controller | the controller's own display |
| 10 | after 9 | read a vendor's C example (an INA226 driver or a Modbus RTU reference); learn to READ C, never write it | `m10-c-port`: port it to Rust | the C program's output on the same inputs |

### Module 1 — `m1-units`

**Drill:** Rustlings `intro`, `variables`, `functions`, `if`, `quiz1`, `primitive_types`.
The tutor names the matching Book section as each group starts, and you read it then:
`variables` → 3.1, `functions` → 3.3, `if` → 3.5, `primitive_types` → 3.2. There is no
separate reading step. Book ch. 1 and ch. 2 stay optional, and `cargo new guessing_game`
is still there if you want it.

**Build:** a binary crate whose `main` checks each function against known values with
`assert_eq!` and prints a line per check.

**Set up (done 2026-11-14):** the `m1-units` crate exists and prints `Hello, world!`;
Rustlings 6.5.0 is in `rustlings/` (run `rustlings` from inside it). Replace the
`Hello, world!` in `m1-units/src/main.rs` with the functions below. Run from the repo
root with `cargo run -p m1-units`. For the Book's ch. 2 guessing game, `cargo new
guessing_game` at the repo root adds it to the workspace.

**Already answered while setting up:** `cargo new` adds the crate to the workspace's
`members` itself; `cargo run` compiles to `target/debug/m1-units` and then runs it, and
`target/` is gitignored; `println!` ends the line, `print!` does not.

Functions to create (bodies are yours):

    fn celsius_to_fahrenheit(c: f64) -> f64        // battery temperature in °F
    fn wh_to_kwh(wh: f64) -> f64                   // energy unit conversion
    fn power_w(volts: f64, amps: f64) -> f64       // P = V × I
    fn charge_state(percent: u8) -> &'static str   // "low" / "ok" / "full" bands

**Decisions left to you:**
1. Where the band edges of `charge_state` sit, and whether 100 is "full" or a separate case.
2. Whether to compare `f64` results exactly or within a tolerance, and why.
3. What `charge_state(150)` should do, given a `u8` allows it.

**Done when:** `cargo run -p m1-units` prints every check passing, and you can explain
why `f64` equality is risky.

### Module 2 — `m2-registers`

**Quiz first:** a closed-book check of about 5 questions (ownership, borrowing, struct kinds,
`&self`), one at a time, plus 3 spaced questions from Module 1. **Read:** Book 4.1, 4.2, 5.1
and 5.3, named when they matter; the matching Rustlings are done.

**Build:** `cargo new m2-registers` at the repo root; `main` checks each function against
hand-computed values with `assert_eq!` and prints a line per check.

**Set up (done 2026-11-18):** the empty `m2-registers` crate exists and prints
`Hello, world!`; `cargo new` added it to the workspace's `members` itself. Run it from the
repo root with `cargo run -p m2-registers`. Scaffolded 2026-11-19: `src/main.rs` holds every
signature below with a `todo!()` body and the cases as comments in `main`.

Functions to create (bodies are yours):

    fn combine_registers(high: u16, low: u16) -> u32   // two 16-bit registers, one 32-bit value
    fn as_signed(raw: u16) -> i16                      // the same 16 bits read as signed
    struct Reading { raw: u16, divisor: u16 }          // a whole number plus a scale
    impl Reading {
        fn new(raw: u16, divisor: u16) -> Self
        fn value(&self) -> f64                         // raw / divisor
    }

Pick hand-computed cases such as: registers that make 65536, the largest `u32`, `65535` read
as signed, and `1263` with divisor `100`.

**Decisions left to you:**
1. Which register is the high half. Modbus fixes byte order inside a register but not the
   order of two registers carrying a 32-bit value, which varies by device (secondary sources;
   confirm with the device manual). How does the name or a comment say which order you use?
2. What `Reading::new` does for a divisor of 0 (`Package::new` panicked; errors come in Module 4).
3. How you compare `value()`, and you write the tolerance check yourself (the repeat-smaller
   task from Module 1).
4. Whether `raw` is `u16` or `i16` when a value can be negative.

**Done when:** `cargo run -p m2-registers` prints every check passing, one tolerance check is
yours, and you can explain why `65535` can mean `-1` and why `value` takes `&self`.

### Module 3 — `m3-signals`

**Drill, in this order** (Rustlings, run from `rustlings/`); the tutor names the Book section
as each group starts:

| # | Group | Book |
|---|---|---|
| 1 | `enums1-3` | 6.1, 6.2 |
| 2 | `strings1-4` | 8.2 |
| 3 | `modules1-3` | 7.1–7.4 |
| 4 | `hashmaps1-3` | 8.3 |
| 5 | `quiz2` | (mixes the groups above) |
| 6 | `options1-3` | 6.1 (`Option`), 6.3 |

**Quiz first** (about 5, one at a time, after the drill): an enum with data and a `match`
that must cover every variant; `String` vs `&str` and why a `String` cannot be indexed;
`pub` and module privacy; `HashMap` `insert`/`get` and why `get` returns an `Option`;
handling an `Option` with `match` or `if let`. Plus 3 spaced from Modules 1–2, after the open
weak spots.

**Build:** `cargo new m3-signals` at the repo root; the tutor scaffolds and explains each item.

Functions to create (bodies are yours):

    enum FunctionCode { ReadHoldingRegisters, ReadInputRegisters, WriteSingleRegister }
    impl FunctionCode {
        fn from_byte(byte: u8) -> Option<FunctionCode>  // 0x03, 0x04, 0x06; anything else None
        fn to_byte(&self) -> u8
        fn is_write(&self) -> bool
    }
    struct Deadband { threshold: f64, last_sent: Option<f64> }
    impl Deadband {
        fn new(threshold: f64) -> Self
        fn should_send(&mut self, value: f64) -> bool   // the first value, then a big enough change
    }
    fn latest_by_register(readings: &[(u16, f64)]) -> HashMap<u16, f64>

Hand-computed cases: `from_byte(0x03)` is `Some(ReadHoldingRegisters)`, `from_byte(0x2B)` is
`None`, `to_byte` round-trips every variant; a deadband of 0.5 over
`12.0, 12.2, 12.6, 12.5, 13.2` sends `12.0, 12.6, 13.2`; `latest_by_register` over
`(100, 12.0), (101, 3.0), (100, 12.5)` gives `{100: 12.5, 101: 3.0}`. The function codes
come from the Modbus application protocol specification (0x03 read holding registers, 0x04
read input registers, 0x06 write single register; an error reply adds 0x80).

**Decisions left to you:**
1. What `from_byte` returns for an unknown byte, and for an error reply such as `0x83`.
2. Whether a change of exactly the threshold is sent (`>` or `>=`), with a check on each side
   of that edge (`12.0` then `12.5` is a difference of exactly 0.5).
3. Why `should_send` takes `&mut self` while `to_byte` takes `&self` (weak spot 7).
4. What `latest_by_register` keeps when one register appears twice.

**Done when:** `cargo run -p m3-signals` prints every check passing, the threshold edge has a
check on both sides, and you can explain why `from_byte` returns an `Option` and why
`should_send` needs `&mut self`.

### Later modules

Each later module's drill, quiz topics, blueprint, decisions and done-when are written here
when the previous module is reviewed, so the plan follows what you actually learned.

### Order from here (2026-11-24)

1. ✅ Build and review `m1-units` (2026-11-18).
2. ✅ Build and review `m2-registers` (2026-11-19).
3. Module 3:
   1. ✅ The drill above, `enums1` through `options3` (2026-11-24).
   2. ✅ The closed-book quiz (2026-11-24, 3 of 5; both misses closed by twins).
   3. 👈 The `m3-signals` build, then the review. Scaffold committed 2026-11-25 (every body `todo!()`); item 1 (`FunctionCode`: `from_byte`, `to_byte`) explained, learner writing it. Still to explain and write: `is_write`, `Deadband`, `latest_by_register`.

### Review checklist

A module is reviewed when every oracle passes, you explain the module's decisions aloud, the
compiler's surprises are in `docs/notes/rust-cheatsheet.md`, and the work is one commit.

### Weak spots (re-ask first)

| # | Idea | Missed | Status |
|---|------|--------|--------|
| 1 | a live reader (`&`) blocks a writer (`push`, `&mut`), error `E0502`: check the line it is created and the line of its LAST use, and look for a write between | 2026-11-18, three times; then right on a twin where the write came after the last use and on a loop that hides the reader (`for x in &v { v.push(*x); }`); 2026-11-19 right (a slice read across `clear`) | closed 2026-11-19 |
| 2 | an integer literal never becomes a float: `3` into an `f64` needs `3.0`, but `n as f64` is a cast you write, so it is fine | 2026-11-18, three times; the twin over-applied the rule to `n as f64`; then right on a two-argument call (`area(3.0, 4)`) and stated the cast rule himself; 2026-11-19 right (a struct field `amps: 3`) | closed 2026-11-19 |
| 3 | lend with `&`, not `&mut`, when a function only reads | 2026-11-18, partial, then right; 2026-11-19 right | closed 2026-11-19 |
| 4 | a value passed or assigned without `&` is moved | 2026-11-18, wrong first, then right twice; 2026-11-19 right (`len` then a move) | closed 2026-11-19 |
| 5 | `&self` borrows, plain `self` uses the value up | 2026-11-18, wrong first, then right twice; 2026-11-19 right | closed 2026-11-19 |
| 6 | `i8` holds -128..=127 | 2026-11-17, right in the 2026-11-18 review; 2026-11-19 right | closed 2026-11-19 |
| 7 | plain `self` MOVES the value (the caller's variable is gone); it does not write. Writing is `&mut self` | 2026-11-19, Module 2 explain question: "transfers ownership from read to write"; then right on a twin (`reset(&mut self)`, `retire(self)`, then a read); later 2026-11-19 wrong again (chose `self` for a method that does `+=` and is called twice; answer `&mut self`); 2026-11-23 right (`Counter`: `finish(self)` on line 3, then a use on line 4); 2026-11-26 framing slipped during the m3 build ("`&self` because I don't want to change self"), then right on a twin (`Meter::close(self)` then `m.kwh` is `E0382`) | reopened 2026-11-26; ask first next session |
| 8 | a `match` must cover every case, and a missing arm is a COMPILE error (`E0004`), not a runtime panic; a guard (`if n > 100`) does not count towards coverage | 2026-11-24, Module 3 quiz Q2: chose "panics at runtime" for a `match` on `Option<u16>` with no arm for `Some(1..)`; then right on a twin (`enum Mode` with no arm for `Off`) | closed 2026-11-24 |
| 9 | `HashMap::get` (and `Vec::pop`) ALWAYS return an `Option`; `{:?}` shows `Some(...)` and `{}` does not compile (`E0277`) | 2026-11-24, Module 3 quiz Q4: chose `2 12.5` (the bare value); then right on a twin (`println!("{}", m.get("a"))` is `E0277`) | closed 2026-11-24 |

Retake 2026-11-19: 6 of 6. The first four questions all had B as the correct letter (a quiz
defect, since fixed); each idea also has a correct answer on an unbiased question.

## Record

| # | Module | Reviewed | Hints | Notes |
|---|---|---|---|---|
| 1 | m1-units | ✅ 2026-11-18 | shape; one reveal (the celsius f64 comment) | quiz 4/5 (missed the `i8` range); 17 checks, every band edge proven by breaking it; Q3 first answered "round to a whole number", corrected to a tolerance; repeat smaller: write the tolerance check yourself in Module 2 |
| 2 | m2-registers | ✅ 2026-11-19 | shape; every body yours; two small reveals (a `65335` typo, the type `u32` passed as an argument) | quizzes 2026-11-18 3/8 and 2/5, retake 2026-11-19 6/6; 8 checks pass, tolerance check yours (`0.0001`); decisions: high register first (documented), divisor 0 panics, `raw: i16`, `divisor` back to `u16` after review; explain 1.5/2 (65535 → -1 right; `self` called a write, weak spot 7) |
| 3 | m3-signals | 👈 build in progress | drill: every body yours; hint ladder to shape only | drill `enums1` through `options3` done 2026-11-24 (`vecs1-2` earlier); quiz 3/5 (missed `match` exhaustiveness and `get` returning `Option`, both closed by twins; Q3 `as i16` of 40000 right, with a 65536 typo, corrected); cheatsheet has Option, `if let`/`while let`, match and module-path notes; scaffold committed 2026-11-25; item 1 done 2026-11-26: `from_byte` and `to_byte` bodies yours and right first time, the checks applied by the tutor on request (a reveal), each caught a mutation; item 2 done 2026-11-26: `is_write` yours, a `_ => false` arm corrected twice (it hides a future write code), final match exhaustive with a `|` read arm, the two checks applied by the tutor on request (a reveal; `assert!` per clippy `bool_assert_comparison`), each caught a mutation; the other items, review and the explain questions still to do |

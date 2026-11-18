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
   **The tutor asks each quiz question ONE at a time through the picker**, with the likely
   answers as options and free text under "Other", and grades with real compiler output after
   the last answer. *(Owner's instruction 2026-11-17: one question per prompt is easier to
   answer.)*
7. **Oracle first:** write the `assert_eq!` before the function body. *(Judgment call.)*
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
| 3 | 5–6 | ch. 6–8; Rustlings enums, modules, vecs, strings, hashmaps | `m3-signals`: Modbus function codes as an enum; a deadband filter | fixtures |
| 4 | 7–8 | ch. 9–10; Rustlings error_handling, generics, traits, lifetimes | `m4-regmap`: parse a register map from CSV | a known map, including bad rows |
| 5 | 9–10 | ch. 11, 13; Rustlings tests, iterators | `m5-summary`: min/max/avg/last/count over a window | hand-computed |
| 6 | 11–12 | ch. 12 (the CLI project) | `m6-contrast`: a Rust port of a WCAG contrast checker | the original Python tool, run on the same public SVGs |
| 7 | 13–14 | ch. 15–16; the Tokio tutorial | `m7-poller`: fake sensor every second → channel → deadband → summary | a deterministic fake sensor |
| 8 | 15–16 | `tokio-modbus` docs | `m8-modbus-tcp`: read and decode registers from a Modbus TCP simulator | the simulator's configured values |
| 9 | 17+ | The Embedded Rust Book (QEMU chapter first) | switch Module 8 to RTU over RS-485 on a real controller | the controller's own display |

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

### Later modules

Each module's functions, decisions and done-when check are written here when the
previous module is reviewed, so the plan follows what you actually learned.

### Order from here (2026-11-17)

1. Build `m1-units` from a blank file with `assert_eq!` oracles, then review it.
2. Write Module 2's blueprint into this plan, build `m2-registers`, review it.
3. Resume Rustlings at `enums1` (Module 3), then `modules`, `strings`, `hashmaps`, and build
   `m3-signals`.

### Review checklist

A module is reviewed when every oracle passes, you explain the module's decisions aloud, the
compiler's surprises are in `docs/notes/rust-cheatsheet.md`, and the work is one commit.

## Record

| # | Module | Reviewed | Hints | Notes |
|---|---|---|---|---|
| 1 | m1-units | ✅ 2026-11-18 | shape; one reveal (the celsius f64 comment) | quiz 4/5 (missed the `i8` range); 17 checks, every band edge proven by breaking it; Q3 first answered "round to a whole number", corrected to a tolerance; repeat smaller: write the tolerance check yourself in Module 2 |
| 2 | m2-registers | ⏳ current | | `move_semantics` and `structs` done early; blueprint to be written into this plan |
| 3 | m3-signals | pending | | `vecs1-2` done early; `enums` onward wait for Modules 1 and 2 |

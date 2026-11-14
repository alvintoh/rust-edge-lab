# Rust course — from zero to an edge telemetry gateway

- **Learner:** new to Rust; experienced software engineer (TypeScript, Go-familiar,
  event-driven integrations).
- **Goal:** by **2027-03-31**, write a Rust program that reads a solar charge controller
  over Modbus and reports what it sees, then move on to embedded Rust on an ESP32.
- **Pace:** about 5–7 hours a week from 2026-11-17; the holiday weeks are buffer.
- **Toolchain:** Rust 1.100.0 (rustup) in WSL and on Windows; Zed in WSL mode.
- **Hardware:** none needed before Module 9 (Jan–Feb purchase).

## How this course works

1. **You write every line.** The tutor gives the shape (signatures, a flow diagram,
   leading questions) and reviews what you wrote; the bodies are yours.
2. **Read, drill, build.** Each module pairs Book chapters with the matching Rustlings
   exercises, then one exercise crate in this workspace.
3. **Every exercise has an oracle**: a hand-computed fixture, a simulator with known
   values, or an existing program to diff against. The borrow checker accepting your code
   proves it is memory-safe, not that it is right.
4. **Small on purpose:** each crate stays under about 500 lines.
5. **One module is reviewed before the next starts**, and each ends in a commit.

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

**Read:** Book ch. 1 (install, hello world, Cargo), ch. 2 (guessing game), ch. 3
(variables, types, functions, control flow). **Drill:** Rustlings `intro`, `variables`,
`functions`, `if`, `primitive_types`.

**Build:** a binary crate whose `main` checks each function against known values with
`assert_eq!` and prints a line per check.

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

## Record

| # | Module | Reviewed | Notes |
|---|---|---|---|

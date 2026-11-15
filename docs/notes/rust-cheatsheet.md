# Rust cheatsheet

My own notes from the course, checked by running them on Rust 1.100.0.

## Integer types: range and sign

- `iN` is signed, `uN` is unsigned. `N` is the bit count: 2^N patterns.
- `iN`: -2^(N-1) to 2^(N-1)-1. `uN`: 0 to 2^N-1.
- "Unsigned" means no sign, so no negatives. It does not mean either sign.
- Same bits, two readings: `11001000` is 200 as `u8`, -56 as `i8`.
- `i8` -128..127 | `i16` -32768..32767 | `u8` 0..255 | `u16` 0..65535.
- Ask the compiler: `i8::MIN`, `i8::MAX`.
- The default for `let x = 5` is `i32`. Use `u` for counts and percentages.

## Shadowing vs `mut`

- `let number = 3;` again makes a new variable that reuses the name, so its type can differ.
- `mut` overwrites the same variable, whose type was fixed at the first `let`.

## Return value: the last expression

- A block's value is its last expression, written WITHOUT a trailing `;`.
- A `;` discards the value, so the block gives `()` instead.
- `fn f() -> i32 { n * n; }` fails: expected `i32`, found `()`.
- No `-> T` in the signature means the function returns `()`.

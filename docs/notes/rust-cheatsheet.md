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

## Where to look things up

- The std reference: https://doc.rust-lang.org/std/index.html (search box at the top).
- A method sits on its type's page: `char::is_alphabetic` is at
  https://doc.rust-lang.org/std/primitive.char.html#method.is_alphabetic
- Offline, matching the installed Rust: `rustup doc --std`
- In Zed: hover a method for its doc comment, or go to definition.

## Slices and `&`

- `&a[1..4]` is a view into `a`: a pointer to the first element plus a length. No copy is made.
- `a[1..4]` alone has type `[i32]`, whose length is not in the type, so a local variable cannot
  hold it. `&` makes it `&[i32]`, always 16 bytes on a 64-bit machine (8 for the address, 8 for the length).
- The `&` does not tell the compiler how long the block is; the length is stored inside the
  reference at runtime.
- The length is `end - start`: `1..4` is 3, `1..5` is 4, `..` is the whole array. It is not `n`
  and not the array's size.
- `1..4` starts at index 1 and stops before index 4: indexes 1, 2, 3.
- An end past the array's length panics at runtime: `range end index 9 out of range for slice of length 5`.

## `vec!` and `[ ]`

- `x[1]` is indexing: the item at position 1. `vec[a]` fails with `cannot find value 'vec'`,
  because it reads as indexing a variable named `vec`.
- `vec![...]` (with the `!`) is a macro that builds a `Vec`, one item per comma-separated thing.
- `vec![b]` with `b = [1, 2, 3]` is `[[1, 2, 3]]`: a list of ONE item, the array.
  `vec![1, 2, 3]` has three items.
- The compiler's hint `use !` fixes the first error but can lead to a type error on the next line.
  Read the next error before trusting a hint.

# Rust cheatsheet

My own notes from the course, checked by running them on Rust 1.100.0.

## Integer types: range and sign

- `iN` is signed, `uN` is unsigned. `N` is the bit count: 2^N patterns. The letters: `i` is
  "integer", which is signed by default as in maths; `u` is "unsigned". There is no `s16`.
- Only the LEFTMOST bit's worth differs between `u16` and `i16` (+32768 vs -32768); every
  other bit is worth the same, and no bit flips. `65535` is all 1s: 32768 + 32767 = 65535 as
  `u16`, -32768 + 32767 = -1 as `i16`.
- `iN`: -2^(N-1) to 2^(N-1)-1. `uN`: 0 to 2^N-1.
- "Unsigned" means no sign, so no negatives. It does not mean either sign.
- Same bits, two readings: `11001000` is 200 as `u8`, -56 as `i8`.
- Two concepts, read off the name: the NUMBER decides storage, the LETTER decides reading.

  | Concept | `u16` | `i16` |
  |---------|-------|-------|
  | storage (bits, patterns) | 16 bits, 65,536 | 16 bits, 65,536 (same) |
  | reading (left bit worth) | +32768 | -32768 (differs) |
  | range (every reading) | 0..=65535 | -32768..=32767 |

- `uN` and `iN` read the SAME bits differently: only the left bit's worth changes (`u8` +128,
  `i8` -128; `u16` +32768, `i16` -32768).
- Range vs one value: `u8` labels the 256 patterns 0..=255 and `i8` labels the same 256
  patterns -128..=127. That is the range. Converting ONE value keeps its pattern.
- `x as i8` / `raw as i16` moves no bits. A value with the left bit 0 (`u8` 0..=127) stays
  the same; one with the left bit 1 (`u8` 128..=255) turns negative: 128 → -128, 200 → -56,
  255 → -1. Same for 16 bits: 32767 stays, 32768 → -32768, 65535 → -1.
- A number outside a type's range can never be stored in it: `let x: i8 = 129;` fails with
  `literal out of range for i8` (the `i8` maximum is 127). Use the next size up, `i16`. There
  is no `i9`: sizes double (8, 16, 32, 64, 128) because memory comes in whole bytes.
  Converting a `u8` 129 with `as i8` does compile, but gives -127 (same bits), not 129.
- By hand: `11111111` is 128+64+32+16+8+4+2+1 = 255 as `u8`, and -128+64+32+16+8+4+2+1 = -1
  as `i8`.
- In an `iN` the leftmost bit is worth -2^(N-1) (`i8`: -128) and the other bits keep their
  normal weights: `10000000` is -128, `11111111` is -1, `01111111` is 127. No minus sign is
  stored (two's complement), and a `u8` with the same bits reads them as 128, 255 and 127.
- The range is lopsided (`-128..=127`) because zero takes one of the non-negative slots.
  `127i8 + 1` overflows: a debug build panics, a release build wraps to `-128`
  (`01111111` + 1 = `10000000`).
- `i8` -128..127 | `i16` -32768..32767 | `u8` 0..255 | `u16` 0..65535.
- Ask the compiler: `i8::MIN`, `i8::MAX`.
- The default for `let x = 5` is `i32`, and for `1.5` it is `f64`. Which type for which job:
  see the table after this section.
- A literal like `8` has no fixed type: it becomes whichever integer type is expected (`u8`,
  `u32`, `i64`), and `300` into a `u8` is a compile error (`literal out of range`). It never
  turns into a float, in a struct field or a `let`: `12` into an `f64` field and
  `let ratio: f64 = 3;` both fail with `expected f64, found integer`; write `12.0` and `3.0`.
- How to check a value going into an `f64` by eye: ask what TYPE it has. A literal with no
  point, or any integer variable, is an integer type and does not fit an `f64`. Add the point
  (`7.0`) or convert in writing (`n as f64`, `f64::from(n)`).

## Which number type for which job

| Job | Type | Why |
|-----|------|-----|
| count, length, index | `usize` | `v.len()` is a `usize` and `v[i]` needs one (an `i32` index is `E0277`); it cannot go below 0, so `2usize - 5` is an overflow |
| a whole number with no other constraint | `i32` | the default |
| whole numbers beyond about 2 billion, or sums that grow | `i64` | an `i32` stops at 2,147,483,647 |
| a known small range (a percentage, a byte, a register) | the smallest `uN` that fits (`u8`, `u16`) | the range is in the type |
| a measurement (volts, temperature, physics) | `f64` | the default float; a tiny rounding error is harmless |
| money | whole cents as an integer, or an exact decimal type | an `f64` stores `0.1` as a close neighbour |

## Shadowing vs `mut`

- `let number = 3;` again makes a new variable that reuses the name, so its type can differ.
- `mut` overwrites the same variable, whose type was fixed at the first `let`.
- `mut` allows any change to what the variable holds: replacing an item (`a[0] = 9`) or
  growing a `Vec` (`v.push(4)`). Without it: `error[E0594]: cannot assign to a[_], as a is
  not declared as mutable`.
- An array can only replace items (its length is in its type); a `Vec` can replace and
  grow. Both need `mut`.

## Return value: the last expression

- A block's value is its last expression, written WITHOUT a trailing `;`.
- A `;` discards the value, so the block gives `()` instead.
- `fn f() -> i32 { n * n; }` fails: expected `i32`, found `()`.
- No `-> T` in the signature means the function returns `()`.

## Printing: `println!` and `{}`

- `println!` is a macro (the `!`): at compile time it pairs each `{}` with the next value after
  the text, in order. `println!("{} {}", 10 + 1, f(2))` evaluates each value, then prints it.
- `{name}` prints a variable by name, with no value after the comma.
- The count is checked when compiling: two `{}` and one value fails with
  `2 positional arguments in format string, but there is 1 argument`.
- A value in `{}` must implement `Display`. Numbers and text do; your own struct does not
  (`E0277: Reading doesn't implement std::fmt::Display`).
- A type is not a value: `f(65535, u32)` fails with `E0423: cannot find value u32`.
- `{}` is the clean, user-facing form and needs `Display`; `{:?}` is the developer form that
  shows the structure and needs `Debug`.

  | Value | `{}` | `{:?}` |
  |-------|------|--------|
  | number, text | `12.4` | `12.4` |
  | `Option` (what `map.get(k)` returns) | `E0277: Option<&f64> doesn't implement Display` | `Some(12.4)` or `None` |
  | a `HashMap` or `Vec` | does not compile | `{"battery_v": 12.4}` |
  | your own struct or enum | does not compile, unless you write `Display` | works after `#[derive(Debug)]` |

- A lookup can fail, so `get` returns `Some(value)` or `None`, not the bare value. To print
  just the number, unwrap it first (`map.get(k).unwrap()`), or handle both cases with `match`
  (`options1-3`).

## Comments: `//`, `///`, `//!`

| Kind | For | Shown by |
|------|-----|----------|
| `//` | notes for whoever edits the code | nobody; tools ignore it |
| `///` | documents the item below it (fn, struct, field) | `cargo doc` and the editor's hover |
| `//!` | documents the whole file, crate or module | `cargo doc`, as the front page |

- A `///` opens with a one-sentence summary of what the item does, then details, then
  sections where they apply: `# Examples`, `# Panics`, `# Errors` (rustdoc book; Rust API
  Guidelines). Questions and to-do notes go in `//`, never `///`.
- An "oracle" is a testing word, not a Rust one: the source of the expected answer, worked
  out without the code under test (by hand, a device display, another program). In
  `assert_eq!(f(1, 0), 65536)` the `65536` is the oracle.

## Unfinished code: `todo!()`

- `todo!()` fits any return type, so the file compiles; calling it panics with
  `not yet implemented` at that line. The std docs: "a placeholder to let your code pass type
  analysis".
- `todo!()` means "I will write this"; `unimplemented!()` means "not provided", maybe on
  purpose.
- Use it only while writing. Finished code has none: it always panics. Clippy's opt-in
  `clippy::todo` lint finds leftovers.

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

## References and borrowing

- `let y = &mut x;` makes `y` a note (an address, 8 bytes on a 64-bit machine) pointing at
  `x`'s data. `y.push(42)` changes the list `x` owns, not `y`.
- At any moment a value has many `&` readers or ONE `&mut` writer, never both and never two
  writers (`E0502`, `E0499`). A growing `Vec` can move to a new block, which would leave a
  second pointer at the old spot.
- A borrow lives from its `let` to its last use, so two `&mut` are fine if the first is
  finished before the second starts.
- A function BORROWS when its parameter has `&` AND the caller writes `&` (`f(&word)`); it
  OWNS when neither does (`f(word)`, which moves `word` away). The two must agree.
- `&` is for reading. To change a borrowed value you need `&mut`. Pass `&` to a function that
  only reads: `&mut` compiles only if the variable is declared `mut` (`E0596` otherwise) and
  asks for write permission the function never uses.
- A reader that is still used later blocks a writer, even a `push` on the very same variable:
  `let r = &s; s.push('b'); println!("{r}");` fails with `E0502` (`push` itself is fine; the
  reader `r` is alive across it). Finish with the reader before the write.
- How to check any `&` by eye: write down the line that creates it and the line that uses it
  LAST. If a write (`push`, `&mut`, an assignment) sits between those two lines, it fails with
  `E0502`. Fixes: use the reader before the write, take the reader after the write, or copy
  the value out (`let copy = v[1];`) so nothing is borrowed.
- `let b = a;` MOVES an owned value such as a `String` or `Vec`: `a` is dead afterwards
  (`E0382`, `borrow of moved value`). Numbers, bools and chars are copied instead, so both
  names stay usable; an owned value needs `.clone()` to be copied.

## Structs: three kinds

- Regular: `struct Reading { volts: f32 }`, made with `Reading { volts: 12.6 }`, read as
  `r.volts`.
- Tuple: `struct Range(i32, i32)`, made with `Range(10, 20)`, read by position as `span.0`.
- Unit: `struct Marker;` has no fields, so it is made by its name alone (`let m = Marker;`).
  It takes 0 bytes and can still implement a trait.
- `..template` (struct update) fills in the fields you leave out: simple ones are copied, an
  owned one like a `String` is MOVED, so the template's field is unusable afterwards unless you
  gave your own value.

## Enums: one choice out of a fixed list

- A struct is AND: every field, every time (leaving one out is `E0063: missing field`).
  An enum is OR: exactly one variant at a time.

  | | `struct` | `enum` |
  |---|----------|--------|
  | holds | all its fields at once | exactly one variant |
  | reads as | volts and amps | Resize or Move or Quit |

- A variant can carry data in the three struct shapes: none (`Quit`), by position
  (`Echo(String)`, `ChangeColor(u8, u8, u8)`), or named (`Resize { width: u64, height: u64 }`).
  Copy the shape `main` uses to make it, with each value replaced by its type.
- Pick one with `Type::Variant`; a name not in the list is `E0599: no variant ... found`.
- `#[derive(Debug)]` lets `{:?}` print it; `{}` does not work on an enum.
- Use a struct when the thing always has all its parts, an enum when it is one of several
  kinds.
- Rust's enum is a "sum type" (one of several shapes); a struct is a "product type" (all of
  its fields). Compared with other languages:

  | Language | What "enum" means | Data per choice? | Every case checked? |
  |----------|-------------------|------------------|---------------------|
  | TypeScript `enum` | named constants (`Red = 0`) | no | no |
  | TypeScript union (`{ kind: "fault"; code: number } \| ...`) | not called enum | yes | only with a `never` check |
  | Go | no enum: `iota` constants, or an interface plus one struct per kind | interface only | no |
  | Rust `enum` | one of several shapes | yes | yes: `match` fails with `E0004` |

## `match`: pick one arm

- `match` looks at a value and runs exactly ONE arm, the first whose pattern fits. In a loop it
  chooses again for every item.
- An arm is `pattern => result,`. For an enum the pattern has the shape that makes the value,
  with NEW variable names where the data goes: `Event::Fault(code) => ...` unpacks the `u16`
  into `code`; `Event::Reading { volts, amps } => ...` unpacks named fields.
- It must cover EVERY case: a missing variant fails with `E0004: non-exhaustive patterns`.
  For numbers or text, finish with `_ => ...` to catch the rest.
- `match` is an expression: it produces a value, so `let result = match op { ... };` works. An
  arm's last line has no `;` (the last-expression rule), and EVERY arm must give the same type
  (`E0308: match arms have incompatible types`).
- A typical shape (`quiz2`): `for (text, cmd) in input { let r = match cmd { A => ..., B(n) => ... };
  out.push(r); }`.
- Methods that return `&str` (like `trim`) need `.to_string()` in an arm whose siblings return
  `String`, so the types agree.

## `Option`: a value that may be missing

- `Option<T>` is an enum with two variants: `Some(value)` (a value is there) and `None`
  (nothing). A function returns it when "no answer" is a normal outcome (`HashMap::get`, a
  lookup, a reading that may not exist). `HashMap::get` and `Vec::pop` ALWAYS return one, even
  when the key or item is there. `Some(5)` and `5` are different types, so
  `assert_eq!(opt, 5)` fails with `E0308: expected Option<u16>, found integer`.
- Get the value out with `.unwrap()`: `Some(4).unwrap()` is `4`, but `None.unwrap()` PANICS
  (`called Option::unwrap() on a None value`). Use it only where `None` cannot happen.
- Make one with `Some(...)` or `None` as the function's last expression, like any other
  value (`options1`).
- Handle both cases with `match` (every case must be covered, as for any enum), or use the
  short forms below when you only care about the `Some` case.

  | Form | Runs | Stops |
  |------|------|-------|
  | `if let Some(v) = opt { ... }` | the block once, if `opt` is `Some` | skips it on `None` |
  | `while let Some(v) = opt_source() { ... }` | the block again and again | the first time it gives `None` |

- The pattern is as deep as the value: `Vec::pop()` on a `Vec<Option<i8>>` returns
  `Option<Option<i8>>`, so the pattern is `Some(Some(n))`. A stored `None` is `Some(None)`,
  which does not match, so the loop stops there (`options2`).

## HashMap: look up by key

- `HashMap<K, V>` finds a value by a key (`"battery_v"` → `12.4`); a `Vec` finds by position.
  `use std::collections::HashMap;`, then `HashMap::new()`. `insert` changes the map, so the
  variable needs `mut` (`E0596` otherwise).
- `insert` on an existing key REPLACES the value; the size does not grow.
- `get(&key)` ALWAYS returns an `Option`: `Some(value)` or `None`, never the bare value, because
  a lookup can fail. Printing it with `{:?}` shows the wrapper (`Some(12.5)`), and `{}` does not
  compile (`Option` has no Display, `E0277`). To use the number, take it out of the `Some`
  (`unwrap`, `if let`, `match`). `contains_key(&key)` is the plain yes/no.
- Add only if missing: `map.entry(key).or_insert(v)` (an existing key is left alone).
- Update from the old value: `*map.entry(key).or_insert(0) += n;`. `or_insert` returns a
  `&mut V`, a borrow of the value inside the map, so `*` reaches the number
  (`E0368: += cannot be applied to &mut u32` without it).
- A struct value that implements `Default` starts at zero with `or_default()`, and its fields
  are changed directly: `map.entry(team).or_default().goals_scored += n;` (no `*` needed).
- A key type must derive `Hash`, `PartialEq` and `Eq` (a key enum, as in `hashmaps2`).
- A map's order is not fixed: sort the entries before printing or comparing.

## Modules: `pub` and `use`

- Everything in a `mod` is private by default. `pub` makes one item visible from outside;
  calling a private one fails with `E0603: function ... is private`. Code in the same module
  can always use its private items. Only add `pub` to what the outside needs.
- A nested module must be `pub` too, or its path is blocked (`E0603: module solar is private`).
- `use` brings a path into scope so you write the short name: `use power::battery::percent;`
  then `percent()` instead of `power::battery::percent()`.
  - `as` renames: `use power::solar::watts as pv_watts;`
  - `{}` groups one path on one line: `use std::time::{SystemTime, UNIX_EPOCH};`
- A `use` only counts inside the module (or file) it is written in: using `percent()` in
  `main` when only `mod panel` has the `use` fails with `E0425: cannot find function`.
- `pub use` re-exports: the name becomes reachable from outside, as `delicious_snacks::fruit`
  in `modules2`. Without the `pub`, `E0603: constant import fruit is private`.
- A module does not see what is outside it unless it asks. An enum written at the top of the
  file and used inside `mod my_module` needs `use super::Command;` there, or it fails with
  `E0425`/`E0433: cannot find type Command in this scope`.
- Paths start from a place:

  | Path | Starts from |
  |------|-------------|
  | `super::Op` | the level ABOVE this module (like `..`) |
  | `self::fruits::PEAR` | this module's own child |
  | `crate::power::battery::percent` | the root of the program (the file), from anywhere |
  | `labels::describe` | a module reached by name from the level it sits in |

  A test module reaches a function in a sibling module through the parent:
  `use super::my_module::transformer;`.
- `use` is not a copy and not a call: it adds a second name for the same item. The standard
  library works the same way: `use std::time::Duration;` brings in `Duration`.

## Text: `&str` vs `String`

- `"text"` is a `&str`: borrowed text in the program (16 bytes: address + length). A `String`
  field owns its text (24 bytes: address, length, capacity).
- Rust will not convert silently (`expected String, found &str`). Write `String::from("text")`,
  `"text".to_string()` or `"text".into()`; each copies the text into memory the `String` owns.
- A text literal is always a `&str` (it does not adapt the way `8` does), and a number is free
  to make while a `String` has to allocate and copy, which is why Rust makes you write it.
- A `String` can grow (`push_str`); a `&str` cannot (`no method named push_str`).
- A function that only reads text takes `&str`, and accepts a literal or a `&String`
  (`f(&word)`): Rust turns `&String` into `&str` for you (deref coercion, Book 15.2).
- A reading function never changes the original; its result comes back as a NEW value:
  `let loud = shout(&word);` leaves `word` as `green` and gives `loud` = `GREEN`.

  | Parameter | Changes the caller's value? | Result comes back as |
  |-----------|-----------------------------|----------------------|
  | `&str` / `&T` | no, read only | a new return value |
  | `&mut String` / `&mut T` | yes, in place | often nothing; the original IS the result |

- Which one to use, by the job (parameters: Book 4.3, established; fields and returns follow
  from ownership):

  | Where | Use | Why |
  |-------|-----|-----|
  | function PARAMETER that only reads | `&str` | takes a literal, a `&String` or a slice; a `String` parameter would refuse a literal and take ownership |
  | struct FIELD that keeps the text | `String` | the struct owns it; a `&str` field fails with `E0106: missing lifetime specifier` until lifetimes (Module 4) |
  | RETURN, text built inside the function | `String` | new text has to be owned by whoever receives it |
  | RETURN, a piece of an input | `&str` | a view into the input, nothing copied (`trim`, `&s[0..1]`) |

- Deciding a value's type by eye (`strings4`): does the expression MAKE new text (`to_string`,
  `String::from`, `to_owned`, `format!`, `replace`, `to_lowercase`) or POINT at existing text
  (a literal, `.trim()`, `&s[a..b]`)? New is `String`, pointing is `&str`, and `.into()` is
  whichever the receiver asks for. Do not pick by length or content: the type is fixed when
  compiling.
- Default: take `&str`, store `String`, return `String` when you built it.

## Methods: what `self` means

- A method lives in `impl Type { ... }`. Its first parameter says how it uses the value:
  `&self` borrows to read, `&mut self` borrows to change, `self` takes ownership and uses it up.
- No `self` at all makes an associated function, called as `Type::new(...)`; `new` is the
  convention for a constructor.
- After a `self` method the original variable is gone (`borrow of moved value`); after `&self`
  or `&mut self` it is still yours.
- `&mut self` is ONE thing, "borrow to write": it changes the caller's value without moving
  ownership. Plain `self` does not write; it TAKES the value.

  | Signature | Meaning | Caller's variable after |
  |-----------|---------|-------------------------|
  | `&self` | borrow to read | still yours |
  | `&mut self` | borrow to read and write | still yours, maybe changed |
  | `self` | take it | gone |

- A `self` method is fine as the LAST use of the value. A number copied out before it survives:
  `let x = b.level(); b.recycle();` then `x` works but `b.level()` fails with `E0382`.
- Picking the signature: is the value used again after the call? Then borrow (`&…`). Does the
  method change a field? Then `&mut`.
- Calling: look at the first parameter. Any `self`: a value, a dot, the name
  (`code.to_byte()`). No `self`: the type, `::`, the name (`FunctionCode::from_byte(0x03)`).
  `code.to_byte()` is shorthand for `FunctionCode::to_byte(&code)`.
- A `Copy` input is never lost. Numbers, `bool` and `char` are `Copy`, built into Rust. Passing
  one by value hands over a copy, so `from_byte(byte: u8)` takes nothing from the caller, and
  `&u8` would gain nothing. Your own type is `Copy` only if its `#[derive]` lists `Clone, Copy`,
  and only when every field is `Copy` (a `String` field gives `E0204`).
- For any input, ask: if I take it by value, does the caller lose it? `Copy`: no, so take it by
  value. Not `Copy` and only read: yes, so borrow (`to_byte(&self)`). Not `Copy` and meant to be
  used up: take it (`close(self)`).
- The return value never decides this. `from_byte` and `to_byte` both hand back a new value;
  they differ only because `u8` is `Copy` and `FunctionCode` is not.

## Formatting and lints

- Indent is 4 spaces, never tabs; lines up to 100 wide (rustfmt's defaults). Never hand-format:
  `cargo fmt` does it, and Zed does it on save.
- Before every commit: `cargo fmt`, then `cargo clippy`. Zed runs clippy on save too.
- `rust-toolchain.toml` pins Rust 1.100.0 with rustfmt and clippy, so the Mac and WSL compile
  the same way.

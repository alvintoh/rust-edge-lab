# Rust cheatsheet

My own notes from the course, checked by running them on Rust 1.100.0.

## Integer types: range and sign

- `iN` is signed, `uN` is unsigned. `N` is the bit count: 2^N patterns.
- `iN`: -2^(N-1) to 2^(N-1)-1. `uN`: 0 to 2^N-1.
- "Unsigned" means no sign, so no negatives. It does not mean either sign.
- Same bits, two readings: `11001000` is 200 as `u8`, -56 as `i8`.
- In an `iN` the leftmost bit is worth -2^(N-1) (`i8`: -128) and the other bits keep their
  normal weights: `10000000` is -128, `11111111` is -1, `01111111` is 127. No minus sign is
  stored (two's complement), and a `u8` with the same bits reads them as 128, 255 and 127.
- The range is lopsided (`-128..=127`) because zero takes one of the non-negative slots.
  `127i8 + 1` overflows: a debug build panics, a release build wraps to `-128`
  (`01111111` + 1 = `10000000`).
- `i8` -128..127 | `i16` -32768..32767 | `u8` 0..255 | `u16` 0..65535.
- Ask the compiler: `i8::MIN`, `i8::MAX`.
- The default for `let x = 5` is `i32`. Use `u` for counts and percentages.
- A literal like `8` has no fixed type: it becomes whichever integer type is expected (`u8`,
  `u32`, `i64`), and `300` into a `u8` is a compile error (`literal out of range`). It never
  turns into a float, in a struct field or a `let`: `12` into an `f64` field and
  `let ratio: f64 = 3;` both fail with `expected f64, found integer`; write `12.0` and `3.0`.

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

## Text: `&str` vs `String`

- `"text"` is a `&str`: borrowed text in the program (16 bytes: address + length). A `String`
  field owns its text (24 bytes: address, length, capacity).
- Rust will not convert silently (`expected String, found &str`). Write `String::from("text")`,
  `"text".to_string()` or `"text".into()`; each copies the text into memory the `String` owns.
- A text literal is always a `&str` (it does not adapt the way `8` does), and a number is free
  to make while a `String` has to allocate and copy, which is why Rust makes you write it.
- A `String` can grow (`push_str`); a `&str` cannot (`no method named push_str`).
- A function that only reads text takes `&str`, and accepts a literal or a `&String`.

## Methods: what `self` means

- A method lives in `impl Type { ... }`. Its first parameter says how it uses the value:
  `&self` borrows to read, `&mut self` borrows to change, `self` takes ownership and uses it up.
- No `self` at all makes an associated function, called as `Type::new(...)`; `new` is the
  convention for a constructor.
- After a `self` method the original variable is gone (`borrow of moved value`); after `&self`
  or `&mut self` it is still yours.

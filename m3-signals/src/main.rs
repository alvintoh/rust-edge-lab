//! Module 3: m3-signals. A Modbus function code, a deadband filter, and the latest value per
//! register.
//!
//! Run from the repo root: `cargo run -p m3-signals`

use std::collections::HashMap;

fn main() {
    // Your checks go here: one `assert_eq!` plus one `println!` per case, as in m2-registers.
    //
    // Hand-computed cases (worked out on paper, so the code is checked against something other
    // than itself):
    //
    //   from_byte(0x03)  is Some(ReadHoldingRegisters)
    //   from_byte(0x2B)  is None
    //   to_byte          round-trips every variant: from_byte(code.to_byte()) is Some(code)
    //
    //   Deadband of 0.5 over 12.0, 12.2, 12.6, 12.5, 13.2 sends 12.0, 12.6, 13.2
    //   Deadband of 0.5, 12.0 then 12.5: a change of exactly 0.5, so check BOTH sides of the edge
    //
    //   latest_by_register(&[(100, 12.0), (101, 3.0), (100, 12.5)]) is {100: 12.5, 101: 3.0}
}

/// The Modbus request kind, the first byte after the device address.
///
/// Source: the Modbus application protocol specification: 0x03 read holding registers, 0x04
/// read input registers, 0x06 write single register.
///
/// `Debug` lets `{:?}` print it; `PartialEq` lets `assert_eq!` compare two of them.
#[derive(Debug, PartialEq)]
enum FunctionCode {
    ReadHoldingRegisters,
    ReadInputRegisters,
    WriteSingleRegister,
}

impl FunctionCode {
    /// Turns the byte read off the wire into a function code.
    ///
    /// Decision 1: what do you return for a byte that is not 0x03, 0x04 or 0x06, such as 0x2B
    /// (a real Modbus code this program does not handle) or 0x83 (0x03 plus 0x80, an ERROR
    /// reply to a read)? Why is `Option` the right return type here, and not a `FunctionCode`
    /// with an `Unknown` variant? Think about what a caller must do with each.
    fn from_byte(byte: u8) -> Option<FunctionCode> {
        todo!()
    }

    /// Turns a function code back into the byte to send.
    ///
    /// Why does this take `&self` and not `self`? What would a caller lose if it took `self`
    /// (weak spot 7 and `options3`)?
    fn to_byte(&self) -> u8 {
        todo!()
    }

    /// True when the request changes the device, false when it only reads.
    ///
    /// Which kind of `match` makes the compiler tell you when a fourth variant is added later?
    fn is_write(&self) -> bool {
        todo!()
    }
}

/// Decides when a changing reading is worth sending, so a value that wobbles by 0.01 does not
/// flood the link.
struct Deadband {
    /// How far a value must move from the last SENT value before it is sent again.
    threshold: f64,
    /// The last value that was sent, or `None` before anything has been sent.
    ///
    /// Why `Option<f64>` and not `f64`? What would you store before the first reading, and why
    /// would `0.0` be a wrong answer?
    last_sent: Option<f64>,
}

impl Deadband {
    /// Makes a filter that has sent nothing yet.
    fn new(threshold: f64) -> Self {
        todo!()
    }

    /// Returns true when `value` should be sent, and remembers it if so.
    ///
    /// Decision 3: why `&mut self` here but `&self` on `to_byte`? Which line changes `self`?
    ///
    /// Decision 2: a change of EXACTLY the threshold (12.0 then 12.5 with a threshold of 0.5):
    /// sent or not? Pick `>` or `>=`, write the choice down, and check both sides of the edge.
    /// (12.0 and 12.5 are both exact in binary, so the difference is exactly 0.5.)
    ///
    /// Compare against the last SENT value, not the previous reading: in the hand-computed run,
    /// 12.2 is 0.2 above 12.0 (not sent) and 12.6 is 0.6 above 12.0 (sent), but 12.6 is only
    /// 0.4 above 12.2. Why does that decide which field you need?
    fn should_send(&mut self, value: f64) -> bool {
        todo!()
    }
}

/// Keeps the newest value for each register, given readings in the order they arrived.
///
/// Decision 4: when a register appears twice, which value do you keep? Which `HashMap` method
/// replaces a value, and which one leaves an existing one alone (see the cheatsheet)?
fn latest_by_register(readings: &[(u16, f64)]) -> HashMap<u16, f64> {
    todo!()
}

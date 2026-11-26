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

    // from_byte: a known byte becomes Some(code); anything else is None.
    assert_eq!(
        FunctionCode::from_byte(0x03),
        Some(FunctionCode::ReadHoldingRegisters)
    );
    println!("from_byte(0x03) = {:?}", FunctionCode::from_byte(0x03));
    assert_eq!(FunctionCode::from_byte(0x2B), None);
    println!("from_byte(0x2B) = {:?}", FunctionCode::from_byte(0x2B));
    assert_eq!(FunctionCode::from_byte(0x83), None);
    println!("from_byte(0x83) = {:?}", FunctionCode::from_byte(0x83));

    // to_byte against the spec: a round trip alone would pass if both functions swapped 0x04 and
    // 0x06 the same way, so pin one byte to its spec value too.
    assert_eq!(FunctionCode::WriteSingleRegister.to_byte(), 0x06);
    println!(
        "WriteSingleRegister.to_byte() = {:#04x}",
        FunctionCode::WriteSingleRegister.to_byte()
    );

    // Round trip: from_byte(code.to_byte()) gives the same code back, for every variant.
    assert_eq!(
        FunctionCode::from_byte(FunctionCode::ReadHoldingRegisters.to_byte()),
        Some(FunctionCode::ReadHoldingRegisters)
    );
    assert_eq!(
        FunctionCode::from_byte(FunctionCode::ReadInputRegisters.to_byte()),
        Some(FunctionCode::ReadInputRegisters)
    );
    assert_eq!(
        FunctionCode::from_byte(FunctionCode::WriteSingleRegister.to_byte()),
        Some(FunctionCode::WriteSingleRegister)
    );
    println!("round trip ok for all 3 variants");

    // is_write: a read leaves the device unchanged, a write changes it.
    assert!(!FunctionCode::ReadHoldingRegisters.is_write());
    println!(
        "ReadHoldingRegisters.is_write() = {}",
        FunctionCode::ReadHoldingRegisters.is_write()
    );
    assert!(FunctionCode::WriteSingleRegister.is_write());
    println!(
        "WriteSingleRegister.is_write() = {}",
        FunctionCode::WriteSingleRegister.is_write()
    );

    // Deadband of 0.5 over 12.0, 12.2, 12.6, 12.5, 13.2: sends 12.0, 12.6, 13.2. Each step is
    // compared with the last SENT value, so 12.6 goes (0.6 from 12.0) though it is 0.4 from 12.2.
    let mut d = Deadband::new(0.5);
    assert!(d.should_send(12.0)); // first reading: always sent
    assert!(!d.should_send(12.2)); // 0.2 from 12.0
    assert!(d.should_send(12.6)); // 0.6 from 12.0
    assert!(!d.should_send(12.5)); // 0.1 from 12.6
    assert!(d.should_send(13.2)); // 0.6 from 12.6
    println!("deadband 0.5 over 12.0, 12.2, 12.6, 12.5, 13.2 sends 12.0, 12.6, 13.2");

    // Two more, so the checks catch two bugs the trace above cannot see:
    // 13.0 is 0.2 from 13.2 (not sent), but 1.0 from 12.0 if the sent value were never stored;
    // 12.5 is 0.7 BELOW 13.2 (sent), but -0.7 without `.abs()`, which is never above 0.5.
    assert!(!d.should_send(13.0));
    assert!(d.should_send(12.5));
    println!("deadband then 13.0 (not sent), 12.5 (0.7 down: sent)");

    // The edge: a change of exactly the threshold (12.0 and 12.5 are exact in binary).
    let mut e = Deadband::new(0.5);
    assert!(e.should_send(12.0));
    assert!(!e.should_send(12.5)); // exactly 0.5: not sent, because should_send uses `>`
    println!("deadband 0.5: 12.0 then 12.5 (exactly 0.5) is not sent");
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
        match byte {
            0x03 => Some(FunctionCode::ReadHoldingRegisters),
            0x04 => Some(FunctionCode::ReadInputRegisters),
            0x06 => Some(FunctionCode::WriteSingleRegister),
            _ => None,
        }
    }

    /// Turns a function code back into the byte to send.
    ///
    /// Why does this take `&self` and not `self`? What would a caller lose if it took `self`
    /// (weak spot 7 and `options3`)?
    fn to_byte(&self) -> u8 {
        match self {
            FunctionCode::ReadHoldingRegisters => 0x03,
            FunctionCode::ReadInputRegisters => 0x04,
            FunctionCode::WriteSingleRegister => 0x06,
        }
    }

    /// True when the request changes the device, false when it only reads.
    ///
    /// Which kind of `match` makes the compiler tell you when a fourth variant is added later?
    fn is_write(&self) -> bool {
        match self {
            FunctionCode::WriteSingleRegister => true,
            FunctionCode::ReadHoldingRegisters | FunctionCode::ReadInputRegisters => false,
        }
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
        Deadband {
            threshold,
            last_sent: None,
        }
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
    ///
    fn should_send(&mut self, value: f64) -> bool {
        match self.last_sent {
            // The first reading is always sent, because there is no last value to compare against.
            None => {
                self.last_sent = Some(value);
                true
            }
            // Compare against the last SENT value, not the previous reading
            Some(last) => {
                // `>` not `>=`: a move of exactly the threshold stays inside the band, so a value
                // bouncing between two points exactly that far apart is not resent every time.
                if (value - last).abs() > self.threshold {
                    self.last_sent = Some(value);
                    true
                } else {
                    false
                }
            }
        }
    }
}

/// Keeps the newest value for each register, given readings in the order they arrived.
///
/// Decision 4: when a register appears twice, which value do you keep? Which `HashMap` method
/// replaces a value, and which one leaves an existing one alone (see the cheatsheet)?
fn latest_by_register(readings: &[(u16, f64)]) -> HashMap<u16, f64> {
    todo!()
}

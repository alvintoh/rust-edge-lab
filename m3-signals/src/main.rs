//! Module 3: m3-signals. A Modbus function code, a deadband filter, and the latest value per
//! register.
//!
//! Run from the repo root: `cargo run -p m3-signals`

use std::collections::BTreeMap;

fn main() {
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

    // latest_by_register: the last value for each register is kept, so 100: 12.5 and 101: 3.0.
    let latest = latest_by_register(&[(100, 12.0), (101, 3.0), (100, 12.5)]);
    assert_eq!(latest[&100], 12.5);
    assert_eq!(latest[&101], 3.0);
    assert_eq!(latest.len(), 2);
    println!(
        "latest_by_register(&[(100, 12.0), (101, 3.0), (100, 12.5)]) = {:?}",
        latest
    );
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
    /// Returns `None` for any other byte: a real code this program does not handle (0x2B), and
    /// an error reply (0x83 is 0x03 plus 0x80). `Option` forces every caller to handle the
    /// unknown case, where an `Unknown` variant could be passed along by mistake.
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
    /// Takes `&self` because it only reads: the caller keeps the code and can use it again.
    fn to_byte(&self) -> u8 {
        match self {
            FunctionCode::ReadHoldingRegisters => 0x03,
            FunctionCode::ReadInputRegisters => 0x04,
            FunctionCode::WriteSingleRegister => 0x06,
        }
    }

    /// True when the request changes the device, false when it only reads.
    ///
    /// Every variant is named, with no `_` arm, so adding a fourth variant is a compile error
    /// here until someone decides whether it writes.
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
    /// The last value that was sent, or `None` before anything has been sent. A starting `0.0`
    /// would be wrong: a first reading within the threshold of 0.0 would never be sent.
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
    /// Takes `&mut self` because it writes `self.last_sent`; `to_byte` only reads, so `&self`.
    ///
    /// A change of exactly the threshold is NOT sent (`>`, not `>=`).
    ///
    /// Compares against the last SENT value, not the previous reading, so slow drift still adds
    /// up: 12.0, 12.2, 12.6 sends 12.6, which is 0.6 from 12.0 though only 0.4 from 12.2.
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
/// `insert` replaces an existing value, so the last reading wins. A `BTreeMap` keeps the
/// registers sorted, so the map prints in the same order on every run.
fn latest_by_register(readings: &[(u16, f64)]) -> BTreeMap<u16, f64> {
    let mut map = BTreeMap::new();
    for &(register, value) in readings {
        map.insert(register, value);
    }
    map
}

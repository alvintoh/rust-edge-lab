//! Module 2: m2-registers. Two 16-bit Modbus registers into one 32-bit value, and a scaled
//! Reading.
//!
//! Run from the repo root: `cargo run -p m2-registers`

fn main() {
    // combine_registers, worked out by hand: the high register counts in units of 65536.
    assert_eq!(combine_registers(1, 0), 65536);
    println!("combine_registers(1, 0) = {}", combine_registers(1, 0));
    assert_eq!(combine_registers(0, 65535), 65535);
    println!(
        "combine_registers(0, 65535) = {}",
        combine_registers(0, 65535)
    );
    assert_eq!(combine_registers(65535, 65535), u32::MAX);
    println!(
        "combine_registers(65535, 65535) = {}",
        combine_registers(65535, 65535)
    );

    // as_signed: the same 16 bits; a value turns negative only when its left bit is 1.
    assert_eq!(as_signed(0), 0);
    println!("as_signed(0) = {}", as_signed(0));
    assert_eq!(as_signed(32767), 32767);
    println!("as_signed(32767) = {}", as_signed(32767));
    assert_eq!(as_signed(32768), -32768);
    println!("as_signed(32768) = {}", as_signed(32768));
    assert_eq!(as_signed(65535), -1);
    println!("as_signed(65535) = {}", as_signed(65535));

    // Reading: an f64 stores 12.63 as a close neighbour, so compare within a tolerance, not ==.
    let r = Reading::new(1263, 100);
    assert!(close_enough(r.value(), 12.63));
    println!("Reading::new(1263, 100).value() = {}", r.value());
}

/// Joins two 16-bit registers into one 32-bit value, `high` first.
///
/// Assumes the device sends the high register first. Modbus does not fix this order, so
/// check the device manual and swap the arguments if it sends the low register first.
fn combine_registers(high: u16, low: u16) -> u32 {
    // `high` becomes a u32 before the multiply: a u16 cannot hold high x 65536.
    (high as u32) * 65536 + (low as u32)
}

/// Reads the same 16 bits as signed: the left bit is worth -32768 instead of +32768.
fn as_signed(raw: u16) -> i16 {
    raw as i16
}

/// Returns true when `got` is within 0.0001 of `want`.
fn close_enough(got: f64, want: f64) -> bool {
    (got - want).abs() < 0.0001
}

/// A whole number plus a scale: the real value is raw / divisor (1263 / 100 = 12.63).
struct Reading {
    /// The register value. Signed, because a measurement such as a battery current can be
    /// negative.
    raw: i16,
    /// The scale from the device manual. Unsigned, because a scale is never negative.
    divisor: u16,
}

impl Reading {
    /// Makes a Reading, called as `Reading::new(1263, 100)`.
    ///
    /// # Panics
    ///
    /// Panics if `divisor` is 0.
    fn new(raw: i16, divisor: u16) -> Self {
        if divisor == 0 {
            panic!("divisor cannot be zero");
        } else {
            Self { raw, divisor }
        }
    }

    /// Returns raw / divisor as an f64.
    fn value(&self) -> f64 {
        // Both convert to f64 before dividing; integer division would drop the .63.
        self.raw as f64 / self.divisor as f64
    }
}

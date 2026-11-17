fn main() {
    // power_w: 12 V x 3 A = 36 W, worked out by hand. 36.0 and 0.0 are exact in f64, so == is safe.
    // 0.1 is NOT exactly representable: power_w(0.1, 3.0) is 0.30000000000000004, so == 0.3
    // fails. For values like that, compare within a small tolerance, or choose exact fixtures.
    assert_eq!(power_w(12.0, 3.0), 36.0);
    println!("ok  power_w(12.0, 3.0) = 36.0");
    assert_eq!(power_w(0.0, 5.0), 0.0);
    println!("ok  power_w(0.0, 5.0) = 0.0");

    // wh_to_kwh: 1500 Wh = 1.5 kWh. 1.5 is exactly representable in f64, so == is safe here.
    assert_eq!(wh_to_kwh(1500.0), 1.5);
    println!("ok  wh_to_kwh(1500.0) = 1.5");
    assert_eq!(wh_to_kwh(0.0), 0.0);
    println!("ok  wh_to_kwh(0.0) = 0.0");

    // celsius_to_fahrenheit: 0 C = 32 F, 100 C = 212 F, -40 C = -40 F.
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    println!("ok  celsius_to_fahrenheit(0.0) = 32.0");
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    println!("ok  celsius_to_fahrenheit(100.0) = 212.0");
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    println!("ok  celsius_to_fahrenheit(-40.0) = -40.0");

    // charge_state: 10% = Low, 50% = Medium, 90% = High, 150% = Invalid percentage.
    assert_eq!(charge_state(10), "Low");
    println!("ok  charge_state(10) = Low");
    assert_eq!(charge_state(50), "Medium");
    println!("ok  charge_state(50) = Medium");
    assert_eq!(charge_state(90), "High");
    println!("ok  charge_state(90) = High");
    assert_eq!(charge_state(150), "Invalid percentage");
    println!("ok  charge_state(150) = Invalid percentage");

    // Edge Low/Medium: 20 is the highest Low and 21 the first Medium. Check BOTH sides of an edge.
    assert_eq!(charge_state(20), "Low");
    println!("ok  charge_state(20) = Low");
    assert_eq!(charge_state(21), "Medium");
    println!("ok  charge_state(21) = Medium");
    assert_eq!(charge_state(80), "Medium");
    println!("ok  charge_state(80) = Medium");
    assert_eq!(charge_state(81), "High");
    println!("ok  charge_state(81) = High");
    assert_eq!(charge_state(100), "High");
    println!("ok  charge_state(100) = High");
    assert_eq!(charge_state(101), "Invalid percentage");
    println!("ok  charge_state(101) = Invalid percentage");
}

fn power_w(volts: f64, amps: f64) -> f64 {
    volts * amps
}

fn wh_to_kwh(wh: f64) -> f64 {
    wh / 1000.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn charge_state(percent: u8) -> &'static str {
    match percent {
        0..=20 => "Low",
        21..=80 => "Medium",
        81..=100 => "High",
        _ => "Invalid percentage",
    }
}

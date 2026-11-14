# rust-edge-lab

Learning Rust by building the small pieces that run on site equipment: decoding Modbus
registers, filtering readings with a deadband, summarising telemetry, and polling a device
asynchronously. The pieces are aimed at energy hardware (solar, batteries, chargers) and
data-centre power monitoring.

This is a **learning lab**: chapter exercises from The Rust Book live alongside the
energy-flavoured ones. Each exercise is a crate in this Cargo workspace, and each has an
**oracle**, a known-correct answer to check against, because code that compiles is not
evidence that its logic is right.

The course plan is [`docs/plans/2026-11-14-rust-course.md`](/docs/plans/2026-11-14-rust-course.md).

## Run an exercise

```
cargo run -p <crate>
cargo test -p <crate>
```

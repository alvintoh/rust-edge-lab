# rust-edge-lab — notes for Claude

**This repo is a Rust course for a learner who is new to Rust.** The learner writes
every line of exercise code. Claude tutors: run `/tutor-mode` against the plan, give the
shape (signatures, a flow diagram, leading questions), answer questions, and review each
module before the next starts. **Never write an exercise body unless the learner picks
"Reveal" or "Apply" for that task.**

- **The plan is the rubric:** `docs/plans/2026-11-14-rust-course.md`. Its Record table
  says which module is current; write the next module's blueprint into the plan only
  after the current one is reviewed.
- **Every exercise has an oracle** (a known-correct answer to check against). Keep each
  crate under about 500 lines.
- **Explain with a concrete example first**, and when explaining code, run it and paste
  the real output.
- **Environment:** the repo lives in WSL (`~/dev/learning/rust-edge-lab`). Run `cargo` and
  `rustlings` there, never from Windows PowerShell. Rust 1.100.0, edition 2024. The
  Rust Book online (doc.rust-lang.org/book) targets 1.98+ with edition 2024, so it matches.
- **Rustlings** lives in `rustlings/`, excluded from the Cargo workspace and committed so
  progress syncs across devices. Run `rustlings` from inside that folder. Its
  `solutions/` folder is for after an exercise passes.
- **This repo is public.** Nothing private goes in it: no salaries, employers or personal
  career plans.

# retris-mb

R(ust) T(etris) for the (m)icro:(b)it v2

You can read the dev diary [here](dev_diary.md).

## Building and Running

### Prerequisites

- Rust stable via [rustup](https://rustup.rs/)
- `thumbv7em-none-eabihf` target for Rust, installed via `rustup target add thumbv7em-none-eabihf`
- [probe-rs](https://probe.rs/) for flashing the micro:bit v2 (successor to `probe-run`), installed via `cargo install probe-run`
  - `cargo-embed` for flashing the micro:bit v2, installed with `probe-rs`

WIP (need to recreate installing from scratch to remember everything)

### Building

```sh
cargo build --release
```

### Running

With the micro:bit v2 plugged in via USB:

```sh
cargo embed --release
```

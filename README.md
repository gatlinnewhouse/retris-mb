# retris-mb

## Gatlin Newhouse

R(ust) T(etris) for the (m)icro:(b)it v2

This is a modified version of the Tetris game for the micro:bit v2. It is written in Rust, and uses the micro:bit v2 inputs depending on which features are enabled. It also uses the micro:bit v2's pixel display to show the game board and falling tetrominos if the pixel display feature is enabled or a screen feature is not used.

I've tested the game with the pixel display, buttons, and logo touch inputs. I test by running it on the micro:bit v2. I have not tested the game with the accelerometer inputs or with an external screen.

In the future I'd like to add the ability to use the accelerometer to control the game, and to use an external screen to display the game board and falling tetrominos with color. This would also allow the tetrominos to be properly shaped rather than modified to fit the 5x5 pixel display on the micro:bit v2. It may also allow score-tracking and other aspects of Tetris which were sacrificed to create a working game on just the micro:bit v2.

The game works by looping while checking for inputs, generating rng seeds, stepping the game state ahead a tick, and checking for cleared rows or a game over.

```rust
loop {
    gal.delay.delay_ms(tick);
    if let Some(true) = gal.buttons.read_a() {
        game.move_left(&mut raster);
    }
    if let Some(true) = gal.buttons.read_b() {
        game.move_right(&mut raster);
    }
    if let Some(true) = gal.logo.read_logo() {
        game.rotate_piece(&mut raster);
    }
    let clr_rows = game.step(&mut raster, seed);
    seed = rng.generate();
    if clr_rows > 0 && clr_rows != 7 {
        // Beep for each row cleared
        repeat_beep(clr_rows, 75u16, &mut gal.delay);
    } else if clr_rows == 7 {
        // Game over
        loop {
            #[cfg(feature = "text")]
            clear_display();
            #[cfg(feature = "text")]
            scroll_text("GAME OVER", &mut gal.delay);
        }
    }
    display_frame(&raster);
}
```

The game step function returns `u8` values to represent whether the game is over or how many rows were cleared. I chose `7u8` as the value for game over when there is no screen present since the pixel display on the micro:bit v2 is only 5x5 pixels large.

Using a fair amount of conditional compilationg based on the Cargo features, I was able to scaffold out the game to work with the built-in display, buttons, and logo touch input. As I continue developing the game I should be able to easily add support for the accelerometer and external screen through the use of Cargo features.

You can read the dev diary [here](dev_diary.md).

The license is MIT, and located in the [LICENSE](LICENSE) file.

### Building and Running

#### Prerequisites

- Rust stable via [rustup](https://rustup.rs/)
- `thumbv7em-none-eabihf` target for Rust, installed via `rustup target add thumbv7em-none-eabihf`
- [probe-rs](https://probe.rs/) for flashing the micro:bit v2 (successor to `probe-run`), installed via `cargo install probe-run`
  - `cargo-embed` for flashing the micro:bit v2, installed with `probe-rs`
- Make sure `rust-lld` is in your `$PATH` for `cargo-embed` to work

#### Documentation

You can generate and read the documentation with `cargo doc --open`.

#### Building

```sh
cargo build --release
```

#### Running

With the micro:bit v2 plugged in via USB:

```sh
cargo embed --release
```

### Videos

Demoing classic controls (buttons and logo touch) with the pixel display on the Micro:bit v2:


# Story of how it was made

## October

First things first, I had to `cargo init` after initializing a new git repo on GitHub. I then needed to add my config files for writing embedded Rust on the micro:bit v2. I shameless ripped these from my older projects for the Embedded Rust course.

Then I copied and modified the magnetometer code from the microbitv2 crate examples directory [here](https://github.com/nrf-rs/microbit/tree/main/examples/magnetometer) to create a branch with proof of concept code for using the accelerometer to control the game, that branch is `accelerometer-poc`.

I asked Bart about the touch logo code and he provided a working example [here](https://github.com/pdx-cs-embedded-rust/mb2-touch) which I copied into the `touch-logo` branch of my repo. Thanks Bart!

Then I copied over my code for button presses from the `breakout` project of the Rust Embedded course to start building `mylib` of board functions for the Tetris game.

I also started working on integrating the `accelerometer-poc` code into `mylib` along with the `touch-logo` code as well. My plan was to have the inputs available as different build configuration features in the crate.

To test getting and reading all the different inputs I created a `"demo"` feature flag which will load all the inputs and print them to the serial console. This is the code that I used to test the inputs and make sure they were working as expected.

I also added a `.editorconfig` to ensure I was using the same formatting `cargo clippy` recommends.

I then developed controls for tilting left/right. I added this to the demo feature to play 1 beep if tilting left or pressing A, and 2 beeps if tilting right or pressing B. I also added a feature to play 3 beeps if the logo is touched.

Then `Inputs` was renamed to `GameAbstractionLayer` to more accurately reflect the purpose of the module as I continue development.

## December

Next I copied in some code for the display, font, and text rendering from the `breakout` project of the Rust Embedded course. I then renamed the `display` module to `pixeldisplay` so that it fit with my current feature naming scheme.

I chose to begin making the tetrominos and using a maximum height of 2 pixels so that each had time to fall and make the game somewhat fun when using the built-in pixel display. This also meant that two full rows would be a clear for the player rather than four rows. This means that our L, S, and T shapes end up being corner tetromino variants or just two pixels on a diagonal.

Then I began assembling the game logic in the `game.rs` file as the `GameState` struct, mirroring what was done in the `breakout` project of the Rust Embedded course. I also added a `GameAbstractionLayer` fields for the display pins and display timer when using the pixel display.

Next I added functions to move left, right, and rotate the falling tetromino. I had to bounds check the tetromino to ensure it didn't go off the screen. An interesting wrinkle of this design is that in order to get a tetromino flush with an edge, you may have to rotate the tetromino. This is because the microbit needs static arrays for pieces and I cannot variably set a tetromino's size on the microbit.

I added a function to drop the pieces and have them fall to the bottom of the screen. Then I worked on clearing full rows, and moving rows down after a row is cleared. I had the speaker beep for how many rows were cleared. I also improved the rng by generating more seeds.

Adding a feature for "text" displaying on the screen allowed me to resolve some clippy warnings with conditional compilation. I added the text feature to display text on the micro:bit v2's screen, to do this I forked parts of [microbit-text](https://github.com/mattheww/microbit-text/)

Next I added a game over check function to see if any columns had all five rows filled, if so I passed `7u8` as a return value back to `main.rs` instead of a number of rows cleared.

Then I added block collision checking into the `drop_piece` function. This is a core mechanic of tetris and after several failed attempts to implement this correctly, it was exciting to see it finally working.

So I started playtesting a bit. Outside of some frustration with pieces being fairly deterministic still, I found that things seemed to be working. I tweaked the tetromino randomness to stop generating so many square tetrominos. I thought maybe we could use the accelerometer to help seed the rng, but I also did not want to give players control over the rng. So I sat on that idea for a bit before deciding whether or not to implement it.

Finally I cleaned up the [README](README.md) file to reflect the listed requirements on Canvas. I only needed to add some testing of the code and a video to the repo to complete the assignment.

Upon some research, it appears that unit testing in a `no_std` environment can be done but requires [some workarounds](https://stackoverflow.com/questions/28185854/how-do-i-test-crates-with-no-std).

I filmed, edited a little, and uploaded a presentation of the game to the repo and added a link to the video in the [README](README.md) file. I used git LFS to upload the video to GitHub.

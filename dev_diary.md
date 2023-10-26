# Story of how it was made

First things first, I had to `cargo init` after initializing a new git repo on GitHub. I then needed to add my config files for writing embedded Rust on the micro:bit v2. I shameless ripped these from my older projects for the Embedded Rust course.

Then I copied and modified the magnetometer code from the microbitv2 crate examples directory [here](https://github.com/nrf-rs/microbit/tree/main/examples/magnetometer) to create a branch with proof of concept code for using the accelerometer to control the game, that branch is `accelerometer-poc`.

I asked Bart about the touch logo code and he provided a working example [here](https://github.com/pdx-cs-embedded-rust/mb2-touch) which I copied into the `touch-logo` branch of my repo. Thanks Bart!

Then I copied over my code for button presses from the `breakout` project of the Rust Embedded course to start building `mylib` of board functions for the Tetris game.

I also started working on integrating the `accelerometer-poc` code into `mylib` along with the `touch-logo` code as well. My plan was to have the inputs available as different build configuration features in the crate.

To test getting and reading all the different inputs I created a `"demo"` feature flag which will load all the inputs and print them to the serial console. This is the code that I used to test the inputs and make sure they were working as expected.

I also added a `.editorconfig` to ensure I was using the same formatting `cargo clippy` recommends.

I then developed controls for tilting left/right. I added this to the demo feature to play 1 beep if tilting left or pressing A, and 2 beeps if tilting right or pressing B. I also added a feature to play 3 beeps if the logo is touched.

Then `Inputs` was renamed to `GameAbstractionLayer` to more accurately reflect the purpose of the module as I continue development.

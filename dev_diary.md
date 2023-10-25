# Story of how it was made

First things first, I had to `cargo init` after initializing a new git repo on GitHub. I then needed to add my config files for writing embedded Rust on the micro:bit v2. I shameless ripped these from my older projects for the Embedded Rust course.

Then I copied and modified the magnetometer code from the microbitv2 crate examples directory [here](https://github.com/nrf-rs/microbit/tree/main/examples/magnetometer) to create a branch with proof of concept code for using the accelerometer to control the game, that branch is `accelerometer-poc`.

I asked Bart about the touch logo code and he provided a working example [here](https://github.com/pdx-cs-embedded-rust/mb2-touch) which I copied into the `touch-logo` branch of my repo. Thanks Bart!

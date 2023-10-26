//! Use the acelorometer as a control for the game
//!
//! As modified from example branch `accelerometer-poc`
//!
//! Original source:
//! <https://github.com/nrf-rs/microbit/blob/main/examples/magnetometer/src/main.rs>
use microbit::{hal::twim, pac::TWIM0};
use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelMode, AccelOutputDataRate, Lsm303agr,
};
use rtt_target::rprintln;

/// Custom typedef for the accelerometer to shorten the typename
type Sensor = Lsm303agr<I2cInterface<twim::Twim<TWIM0>>, MagOneShot>;

pub struct Accel {
    /// Accelerometer sensor
    pub accel: Sensor,
}

impl Accel {
    /// Set up the accelerometer
    ///
    /// # Arguments
    /// * `i2c` - The i2c interface for TWIM peripheral
    ///
    /// # Returns
    /// * `Self` - The accelerometer as a struct
    pub fn new(i2c: twim::Twim<TWIM0>) -> Self {
        let mut sensor = Lsm303agr::new_with_i2c(i2c);
        match sensor.accelerometer_id() {
            Ok(0x33u8) => {}
            _ => rprintln!("accelerometer not found"),
        }
        sensor.init().unwrap();
        sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

        rprintln!("normal mode");
        sensor.set_accel_mode(AccelMode::Normal).unwrap();
        Self { accel: sensor }
    }

    /// Read the accelerometer
    ///
    /// # Returns
    /// * `Option<(i32, i32, i32)>` - The x, y, and z values of the accelerometer
    pub fn read_accel(&mut self) -> Option<(i32, i32, i32)> {
        if self.accel.accel_status().unwrap().xyz_new_data {
            let data = self.accel.accel_data().unwrap();
            return Some((data.x, data.y, data.z));
        }
        None
    }
}

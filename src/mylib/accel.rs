//! Accelorometer sensor as modified from example branch `accelerometer-poc`
use microbit::{hal::twim, pac::TWIM0};

use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelMode, AccelOutputDataRate, Lsm303agr,
};
use rtt_target::rprintln;

type Sensor = Lsm303agr<I2cInterface<twim::Twim<TWIM0>>, MagOneShot>;

pub struct Accel {
    /// Accelerometer
    pub accel: Sensor,
}

impl Accel {
    /// Set up the accelerometer
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
    pub fn read_accel(&mut self) -> Option<(i32, i32, i32)> {
        if self.accel.accel_status().unwrap().xyz_new_data {
            let data = self.accel.accel_data().unwrap();
            return Some((data.x, data.y, data.z));
        }
        None
    }
}

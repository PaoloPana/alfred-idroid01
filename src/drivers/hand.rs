use crate::{generate_getter, generate_setter, generate_sleep};

const HAND: u16 = 0x2d;

const PLIERS_REG: u8 = 0x01;
const STRENGTH_CLOSE_REG: u8 = 0x02;
const STRENGTH_OPEN_REG: u8 = 0x03;
const STRENGTH_LEVEL_REG: u8 = 0x04;
const VOLTAGE_LEVEL_REG: u8 = 0x5;
const ERROR_TYPE_REG: u8 = 0x06;

pub struct Hand {
    path: String,
    device_id: u16
}

impl Hand {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.into(),
            device_id: HAND
        }
    }

    generate_getter!(get_pan_pos, PLIERS_REG);
    generate_getter!(get_strength_close, STRENGTH_CLOSE_REG);
    generate_getter!(get_strength_open, STRENGTH_OPEN_REG);
    generate_getter!(get_strength_level, STRENGTH_LEVEL_REG);
    generate_getter!(get_voltage_level, VOLTAGE_LEVEL_REG);
    generate_getter!(get_error_type, ERROR_TYPE_REG);

    generate_setter!(set_pan_pos, PLIERS_REG);
    generate_setter!(set_strength_close, STRENGTH_CLOSE_REG);
    generate_setter!(set_strength_open, STRENGTH_OPEN_REG);
    generate_setter!(set_strength_level, STRENGTH_LEVEL_REG);
    generate_setter!(set_voltage_level, VOLTAGE_LEVEL_REG);

    generate_sleep!();
}

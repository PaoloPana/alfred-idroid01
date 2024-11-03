use std::collections::LinkedList;
use crate::drivers::results::TiltPan;
use crate::{
    generate_boot, generate_get_firmware, generate_getter,
    generate_getter_map, generate_setter,
    generate_sleep, generate_getter_str
};

const HEAD: u16 = 0x29;
const TILT_REG_UP: u8 = 0x01;
const TILT_REG_DOWN: u8 = 0x02;
const PAN_REG_RIGHT: u8 = 0x03;
const PAN_REG_LEFT: u8 = 0x04;
const LED_REG_ON: u8 = 0x05;
const LED_REG_OFF: u8 = 0x06;
const BLINK_REG_ON: u8 = 0x07;
const BB_REG: u8 = 0x08;
const TOUCH_SF_REG: u8 = 0x09;
const TILTPOS_REG: u8 = 0x0b;
const PANPOS_REG: u8 = 0x0c;

pub struct Head {
    path: String,
    device_id: u16
}

impl Head {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.into(),
            device_id: HEAD
        }
    }

    generate_getter_map!(get_tilt, TILT_REG_UP, TiltPan::from_u8, TiltPan);
    generate_getter_map!(get_pan, PAN_REG_RIGHT, TiltPan::from_u8, TiltPan);
    generate_getter!(get_led, LED_REG_ON);
    generate_getter!(get_blink, BLINK_REG_ON);
    generate_getter!(get_bb, BB_REG);
    generate_getter!(get_touch_sf, TOUCH_SF_REG);
    generate_getter!(get_tilt_pos, TILTPOS_REG);
    generate_getter!(get_pan_pos, PANPOS_REG);

    generate_setter!(set_tilt_up, TILT_REG_UP);
    generate_setter!(set_tilt_down, TILT_REG_DOWN);
    generate_setter!(set_pan_right, PAN_REG_RIGHT);
    generate_setter!(set_pan_left, PAN_REG_LEFT);
    generate_setter!(set_led_on, LED_REG_ON);
    generate_setter!(set_led_off, LED_REG_OFF);
    generate_setter!(set_blink, BLINK_REG_ON);
    generate_setter!(set_bb, BB_REG);

    generate_boot!();
    generate_sleep!();
    generate_get_firmware!();

    pub fn set_led_status(&self, led_status: u8) -> Result<(), String> {
        self.set_led_on(led_status)?;
        self.set_led_off(255-led_status)?;
        Ok(())
    }

    pub fn get_status(&self) -> String {
        format!("Head (firmware: {})\n - Tilt: {}\n - Pan: {}\n - LED: {}\n - Blink: {}\n - BB: {}\n - Touch-SF: {}\n - Tilt POS: {}\n - Pan POS: {}",
                self.get_firmware().unwrap_or_else(|e| e),
                self.get_tilt().map_or_else(|e| e, |v| v.to_string()),
                self.get_pan().map_or_else(|e| e, |v| v.to_string()),
                self.get_led().map_or_else(|e| e, |v| v.to_string()),
                self.get_blink().map_or_else(|e| e, |v| v.to_string()),
                self.get_bb().map_or_else(|e| e, |v| v.to_string()),
                self.get_touch_sf().map_or_else(|e| e, |v| v.to_string()),
                self.get_tilt_pos().map_or_else(|e| e, |v| v.to_string()),
                self.get_pan_pos().map_or_else(|e| e, |v| v.to_string())
        )
    }

    pub fn get_command(&self, mut commands: LinkedList<String>) -> Result<String, String> {
        if commands.is_empty() {
            return Err("Command not found".to_string());
        }
        let command = commands.pop_front().ok_or("Command not found")?;
        let param = commands.pop_front();
        match param {
            None => {
                match command.as_str() {
                    "" => Ok(self.get_status()),
                    "tilt" => self.get_tilt().map(|v| v.to_string()),
                    "pan" => self.get_pan().map(|v| v.to_string()),
                    "led" => self.get_led().map(|v| v.to_string()),
                    "blink" => self.get_blink().map(|v| v.to_string()),
                    "touch" => self.get_touch_sf().map(|v| v.to_string()),
                    "tilt_pos" => self.get_tilt_pos().map(|v| v.to_string()),
                    "firmware" => self.get_firmware(),
                    _ => Err("Unknown".to_string())
                }
            },
            Some(param) => {
                let param = param.parse::<u8>().map_err(|e| e.to_string())?;
                match command.as_str() {
                    "" => Ok(self.get_status()),
                    "tilt" => self.set_tilt_up(param).map(|()| "OK".to_string()),
                    "pan" => self.set_pan_right(param).map(|()| "OK".to_string()),
                    "led" => self.set_led_status(param).map(|()| "OK".to_string()),
                    "blink" => self.set_blink(param).map(|()| "OK".to_string()),
                    _ => Err("Unknown".to_string())
                }
            }
        }
    }
}
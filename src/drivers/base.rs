use crate::*;

const BASE: u16 = 0x2a;
const FWD_REG: u8 = 0x01;
const BWD_REG: u8 = 0x02;
const ROT_FW_REG: u8 = 0x03;
const ROT_BW_REG: u8 = 0x04;
const BB_REG: u8 = 0x05;
const US1_REG: u8 = 0x06;
const US2_REG: u8 = 0x07;
const US3_REG: u8 = 0x08;
const BASE_REG: u8 = 0x09;
const LEFT_SPEED_REG: u8 = 0x0b;
const RIGHT_SPEED_REG: u8 = 0x0c;
const LEFT_COUNT_REG: (u8, u8) = (0x0d, 2);
const NOTCHES_LW_REG: (u8, u8) = (0x0d, 2);
const RIGHT_COUNT_REG: (u8, u8) = (0x0f, 2);
const NOTCHES_RW_REG: (u8, u8) = (0x0f, 2);

pub struct Base {
    path: String,
    device_id: u16
}

impl Base {
    pub fn new(path: &str) -> Base {
        Base {
            path: path.into(),
            device_id: BASE
        }
    }

    generate_getter!(get_fwd, FWD_REG);
    generate_getter!(get_bwd, BWD_REG);
    generate_getter!(get_rot_fw, ROT_FW_REG);
    generate_getter!(get_rot_bw, ROT_BW_REG);
    generate_getter!(get_bb, BB_REG);
    generate_getter!(get_us1, US1_REG);
    generate_getter!(get_us2, US2_REG);
    generate_getter!(get_us3, US3_REG);
    generate_getter!(get_base, BASE_REG);
    generate_getter_block!(get_notches_lw, NOTCHES_LW_REG);
    generate_getter_block!(get_notches_rw, NOTCHES_RW_REG);

    generate_setter!(set_fwd, FWD_REG);
    generate_setter!(set_bwd, BWD_REG);
    generate_setter!(set_rot_fw, ROT_FW_REG);
    generate_setter!(set_rot_bw, ROT_BW_REG);
    generate_setter!(set_bb, BB_REG);
    generate_setter!(set_base, BASE_REG);
    generate_setter!(set_left_speed, LEFT_SPEED_REG);
    generate_setter!(set_right_speed, RIGHT_SPEED_REG);

    // TODO: implement set_left_count and set_right_count

    generate_sleep!();
    generate_boot!();
    generate_get_firmware!();

    pub fn get_status(&self) -> String {
        format!("BASE (firmware: {})\n - get_fwd: {}\n - get_bwd: {}\n - get_rot_fw: {}\n - get_rot_bw: {}\n - BB: {}\n - get_base: {}", //\n - get_notches_lw: {}\n - get_notches_rw: {}",
                self.get_firmware().unwrap_or_else(|e| e),
                self.get_fwd().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_bwd().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_rot_fw().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_rot_bw().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_bb().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_base().map(|v| v.to_string()).unwrap_or_else(|e| e)/*,
                self.get_notches_lw().map(|v| v.to_string()).unwrap_or_else(|e| e),
                self.get_notches_rw().map(|v| v.to_string()).unwrap_or_else(|e| e)*/
        )
    }

}

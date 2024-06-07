use crate::*;

const ARMS: u16 = 0x2c;
const LEFT_ARM_REG: u8 = 0x01;
const RIGHT_ARM_REG: u8 = 0x02;
const LIGHTS_REG: u8 = 0x03;
const TEMP_REG: u8 = 0x04;
const GPIO_STAT_REG: u8 = 0x05;
const GPIO_INT_REG: u8 = 0x06;
const GPIO_ON_REG: u8 = 0x05;
const GPIO_OFF_REG: u8 = 0x06;
const GPIO_DD_REG: u8 = 0x07;
const GPIO_A1_REG: u8 = 0x08;
const GPIO_A2_REG: u8 = 0x09;
const LEFT_TOOL_REG: u8 = 0x0b;
const RIGHT_TOOL_REG: u8 = 0x0c;
const LEFT_POS_REG: u8 = 0x0d;
const RIGHT_POS_REG: u8 = 0x0e;
const GPIO_TYPE_REG: u8 = 0x0f;
const WAVE_GEN_REG: (u8, u8) = (0x10, 2);

pub struct Arms {
    path: String,
    device_id: u16
}

impl Arms {
    pub fn new(path: &str) -> Arms {
        Arms {
            path: path.into(),
            device_id: ARMS
        }
    }

    generate_getter!(get_left_arm, LEFT_ARM_REG);
    generate_getter!(get_right_arm, RIGHT_ARM_REG);
    generate_getter!(get_lights, LIGHTS_REG);
    generate_getter!(get_temp, TEMP_REG);
    generate_getter!(get_gpio_stat, GPIO_STAT_REG);
    generate_getter!(get_gpio_int, GPIO_INT_REG);
    generate_getter!(get_gpio_dd, GPIO_DD_REG);
    generate_getter!(get_gpio_a1, GPIO_A1_REG);
    generate_getter!(get_gpio_a2, GPIO_A2_REG);
    generate_getter!(get_left_tool, LEFT_TOOL_REG);
    generate_getter!(get_right_tool, RIGHT_TOOL_REG);
    generate_getter!(get_left_pos, LEFT_POS_REG);
    generate_getter!(get_right_pos, RIGHT_POS_REG);
    generate_getter!(get_gpio_type, GPIO_TYPE_REG);
    generate_getter_block!(get_wave_gen, WAVE_GEN_REG);

    generate_setter!(set_left_arm, LEFT_ARM_REG);
    generate_setter!(set_right_arm, RIGHT_ARM_REG);
    generate_setter!(set_lights, LIGHTS_REG);
    generate_setter!(set_gpio_on, GPIO_ON_REG);
    generate_setter!(set_gpio_off, GPIO_OFF_REG);
    generate_setter!(set_gpio_dd, GPIO_DD_REG);
    generate_setter!(set_left_tool, LEFT_TOOL_REG);
    generate_setter!(set_right_tool, RIGHT_TOOL_REG);
    generate_setter!(set_gpio_type, GPIO_TYPE_REG);

    //generate_getter_block!(get_wave_gen, WAVE_GEN_REG); // TODO: setter

    generate_sleep!();
    generate_boot!();
    generate_get_firmware!();
}
use i2cdev::{core::I2CDevice, linux::LinuxI2CDevice};

pub const SLEEP_REG: u8 = 0x0a;
pub const BOOT_REG: u8 = 0xaa;
pub const FIRMWARE_REG: (u8, u8) = (0xaa, 8);

#[macro_export]
macro_rules! generate_getter {
    ($name:ident, $register: expr) => {
        pub fn $name(&self) -> Result<u8, String> {
            return crate::drivers::driver::read_register(self.path.clone(), self.device_id, $register);
        }
    };
}
#[macro_export]
macro_rules! generate_getter_map {
    ($name:ident, $register: expr, $mapper: expr, $ret_type: ty) => {
        pub fn $name(&self) -> Result<$ret_type, String> {
            return crate::drivers::driver::read_register(self.path.clone(), self.device_id, $register)
            .map($mapper);
        }
    };
}

#[macro_export]
macro_rules! generate_getter_str {
    ($name:ident, $register: expr) => {
        pub fn $name(&self) -> Result<String, String> {
            return crate::drivers::driver::read_register_str(self.path.clone(), self.device_id, $register.0, $register.1);
        }
    };
}

#[macro_export]
macro_rules! generate_getter_block {
    ($name:ident, $register: expr) => {
        pub fn $name(&self) -> Result<Vec<u8>, String> {
            return crate::drivers::driver::read_register_block(self.path.clone(), self.device_id, $register.0, $register.1);
        }
    };
}

#[macro_export]
macro_rules! generate_setter {
    ($name:ident, $register: expr) => {
        pub fn $name(&self, value: u8) -> Result<(), String> {
            return crate::drivers::driver::write_register(self.path.clone(), self.device_id, $register, value);
        }
    };
}

#[macro_export]
macro_rules! generate_get_firmware { () => { generate_getter_str!(get_firmware, crate::drivers::driver::FIRMWARE_REG); }; }

#[macro_export]
macro_rules! generate_boot { () => { generate_setter!(boot, crate::drivers::driver::BOOT_REG); }; }

#[macro_export]
macro_rules! generate_sleep { () => { generate_setter!(sleep, crate::drivers::driver::SLEEP_REG); }; }

pub fn read_register_str(path: String, device: u16, register: u8, len: u8) -> Result<String, String> {
    return read_register_block(path, device, register, len)
        .and_then(|res| String::from_utf8(res).map_err(|_| "UTF8 error".to_string()));
}

pub fn connect(path: String, device: u16) -> Result<LinuxI2CDevice, String> {
    return LinuxI2CDevice::new(path, device)
        .map_err(|_| "Connection error".to_string());
}

pub fn read_register_block(path: String, device: u16, register: u8, len: u8) -> Result<Vec<u8>, String> {
    return connect(path, device)?
        .smbus_read_i2c_block_data(register, len)
        .map_err(|_| "Read block error".to_string());
}

pub fn read_register(path: String, device: u16, register: u8) -> Result<u8, String> {
    return connect(path, device)?
        .smbus_read_byte_data(register)
        .map_err(|_| "Read error".to_string());
}

pub fn write_register(path: String, device: u16, register: u8, value: u8) -> Result<(), String> {
    return connect(path, device)?
        .smbus_write_byte_data(register, value)
        .map_err(|_| "Write error".to_string());
}

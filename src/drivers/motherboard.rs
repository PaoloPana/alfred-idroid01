use crate::{LinkedList, generate_boot, generate_get_firmware, generate_getter, generate_getter_str, generate_setter, generate_sleep};

const MOTHERBOARD: u16 = 0x2b;
const MIND_REG: u8 = 0x07;
const BODY_REG: u8 = 0x08;
const KBD_REG: u8 = 0x0e;

pub struct Motherboard {
    path: String,
    device_id: u16
}

impl Motherboard {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.into(),
            device_id: MOTHERBOARD
        }
    }

    generate_getter!(get_mind, MIND_REG);
    generate_getter!(get_body, BODY_REG);
    generate_getter!(get_kbd, KBD_REG);

    generate_sleep!();
    generate_boot!();
    generate_get_firmware!();

    pub fn get_status(&self) -> String {
        format!("MOTHERBOARD (firmware: {})\n - get_mind: {}\n - get_body: {}\n - get_kbd: {}",
                self.get_firmware().unwrap_or_else(|e| e),
                self.get_mind().map_or_else(|e| e, |v| v.to_string()),
                self.get_body().map_or_else(|e| e, |v| v.to_string()),
                self.get_kbd().map_or_else(|e| e, |v| v.to_string())
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
                    "mind" => self.get_mind().map(|v| v.to_string()),
                    "body" => self.get_body().map(|v| v.to_string()),
                    "kbd" => self.get_kbd().map(|v| v.to_string()),
                    "firmware" => self.get_firmware(),
                    _ => Err("Unknown".to_string())
                }
            },
            Some(_) => {
                Err("Unknown".to_string())
            }
        }
    }

}

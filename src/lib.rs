pub mod drivers;
pub mod commands;

use std::collections::LinkedList;
use crate::drivers::arms::Arms;
use crate::drivers::base::Base;
use crate::drivers::hand::Hand;
use crate::drivers::head::Head;

pub struct Drivers {
    pub head: Head,
    pub base: Base,
    pub arms: Arms,
    pub hand: Hand
}
impl Drivers {
    pub fn new(path: &str) -> Drivers {
        Drivers {
            head: Head::new(path),
            base: Base::new(path),
            arms: Arms::new(path),
            hand: Hand::new(path)
        }
    }

    pub fn get_head(&self) -> &Head {
        &self.head
    }

    pub fn get_command(&self, command: String) -> Result<String, String> {
        let mut commands = command.split(" ").map(|s| s.to_string()).collect::<LinkedList<String>>();
        if commands.len() == 0 {
            return Err("Error: empty command".to_string());
        }
        let first_command = commands.pop_front().unwrap();
        match first_command.as_str() {
            "" => Err("Error: empty command".to_string()),
            // TODO: continue implementing
            "head" => self.head.get_command(commands).map_err(|_| format!("Unknown command {}", command)),
            _ => Err(format!("Unknown command {}", command))
        }
    }
}

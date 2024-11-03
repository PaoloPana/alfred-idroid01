pub mod drivers;

use std::collections::LinkedList;
use crate::drivers::arms::Arms;
use crate::drivers::base::Base;
use crate::drivers::hand::Hand;
use crate::drivers::head::Head;
use crate::drivers::motherboard::Motherboard;

pub struct Drivers {
    pub head: Head,
    pub base: Base,
    pub arms: Arms,
    pub hand: Hand,
    pub motherboard: Motherboard
}
impl Drivers {
    pub fn new(path: &str) -> Self {
        Self {
            head: Head::new(path),
            base: Base::new(path),
            arms: Arms::new(path),
            hand: Hand::new(path),
            motherboard: Motherboard::new(path)
        }
    }

    pub const fn get_head(&self) -> &Head {
        &self.head
    }

    pub fn get_command(&self, command: &str) -> Result<String, String> {
        let mut commands = command.split(' ').map(ToString::to_string).collect::<LinkedList<String>>();
        if commands.is_empty() {
            return Err("Error: empty command".to_string());
        }
        let first_command = commands.pop_front().ok_or_else(|| format!("Error: invalid command: {command}"))?;
        match first_command.as_str() {
            "" => Err("Error: empty command".to_string()),
            "head" => self.head.get_command(commands),
            "base" => self.base.get_command(commands),
            // TODO: implement other commands
            //"arms" => self.arms.get_command(commands).map_err(|_| format!("Unknown command {}", command)),
            //"hand" => self.hand.get_command(commands).map_err(|_| format!("Unknown command {}", command)),
            "motherboard" => self.motherboard.get_command(commands),
            _ => Err(format!("Unknown command {command}"))
        }
    }
}

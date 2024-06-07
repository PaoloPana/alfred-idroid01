pub mod drivers;

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
}

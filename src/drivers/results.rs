use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
pub enum TiltPan {
    InMovement,
    TiltError,
    PanError,
    Unknown(u8)
}
impl TiltPan {
    pub fn from_u8(val: u8) -> TiltPan {
        match val {
            0x01 => TiltPan::InMovement,
            0xAA => TiltPan::TiltError,
            0xFF => TiltPan::PanError,
            v => TiltPan::Unknown(v)
        }
    }
}
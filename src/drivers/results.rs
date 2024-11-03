use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
pub enum TiltPan {
    InMovement,
    TiltError,
    PanError,
    Unknown(u8)
}
impl TiltPan {
    pub const fn from_u8(val: u8) -> Self {
        match val {
            0x01 => Self::InMovement,
            0xAA => Self::TiltError,
            0xFF => Self::PanError,
            v => Self::Unknown(v)
        }
    }
}
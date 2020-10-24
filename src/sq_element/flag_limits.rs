const MAX_NUM: u8 = 9;

/// Sets limits for types that are used as Integers
pub trait ZeroAndOne{
    const ZERO: Self;
    const ONE : Self;
}
impl ZeroAndOne for u8{
    const ONE: Self = 1u8;
    const ZERO: Self = 0u8;
}
impl ZeroAndOne for u16{
    const ONE: Self = 1u16;
    const ZERO: Self = 0u16;
}
impl ZeroAndOne for u32{
    const ONE: Self = 1u32;
    const ZERO: Self = 0u32;
}
pub trait IntLimits: ZeroAndOne {
    const VMAX:  Self;
}
impl IntLimits for u8 {
    const VMAX: Self = MAX_NUM;
}
impl IntLimits for u16 {
    const VMAX: Self = MAX_NUM as u16;
}

pub trait FlagLimits: ZeroAndOne {
    const VMAX: Self;
    const FMAX: Self;
}
impl FlagLimits for u16 {
    const VMAX: Self = 0b100000000;
    const FMAX: Self = 0b111111111;
}

impl FlagLimits for u32 {
    const VMAX: Self = 0b1000000000000000;
    const FMAX: Self = 0b1111111111111111;
}
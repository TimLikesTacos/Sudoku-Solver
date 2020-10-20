const MAX_NUM: u8 = 9;

/// Sets limits for types that are used as Integers
pub trait IntLimits {
    const VMAX:  Self;
    const VONE:   Self;
    const ZERO:  Self;
}
impl IntLimits for u8 {
    const VMAX: Self = MAX_NUM;
    const VONE : Self = 1u8;
    const ZERO: Self = 0u8;
}
impl IntLimits for u16 {
    const VMAX: Self = MAX_NUM as u16;
    const VONE : Self = 1u16;
    const ZERO: Self = 0u16;
}

pub trait FlagLimits {
    const VMAX: Self;
    const FMAX: Self;
    const ONE : Self;
    const ZERO: Self;
}
impl FlagLimits for u16 {
    const VMAX: Self = 0b100000000;
    const FMAX: Self = 0b111111111;
    const ONE : Self = 1u16;
    const ZERO: Self = 0u16;
}


impl FlagLimits for u32 {
    const VMAX: Self = 0b1000000000000000;
    const FMAX: Self = 0b1111111111111111;
    const ONE : Self = 1u32;
    const ZERO: Self = 0u32;
}
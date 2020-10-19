use crate::flag::{FlagType, ValueTr, Flag};
use std::convert::TryFrom;
use crate::square::IncReset;

/**
todo: macro-ize this to allow for different primitive integers
**/
impl From<usize> for FlagType<u16> {
    fn from(item: usize) -> FlagType<u16> {
        if item == 0 {
            FlagType { flags: 0, count: 0 }
        } else {
            FlagType {
                flags: 1 << (item - 1),
                count: 1,
            }
        }
    }
}

impl From<FlagType<u16>> for usize {
    fn from(item: FlagType<u16>) -> usize {
        if item.count != 1 {
            0
        } else {
            (item.flags.trailing_zeros() + 1) as usize
        }

        // let mut v: u16 = item.flags;
        // let mut num: u16 = 0;
        // let mut place = 1;
        // while v > 0 {
        //     if v & 1 > 0 {
        //         num += place;
        //     }
        //     v >>= 1;
        //     place += 1;
        // }
        // num as usize
    }
}

impl From<u16> for FlagType<u16> {
    fn from(item: u16) -> FlagType<u16> {
        if item == 0 {
            FlagType { flags: 0, count: 0 }
        } else {
            FlagType {
                flags: 1 << (item - 1),
                count: 1,
            }
        }
    }
}

impl From<FlagType<u16>> for u16 {
    fn from(item: FlagType<u16>) -> u16 {
        if item.count != 1 {
            0
        } else {
            /*
            todo: proper error handling
             */
            u16::try_from(item.flags.trailing_zeros() + 1).unwrap()
        }

    }
}





impl <F: Flag> IncReset<F> for FlagType<F> {
    fn inc (&mut self) -> bool {
        if self.fixed || self.value == F::VMAX {
            false
        } else {
            let old = self.value;
            self.value = old << 1;
            true
        }
    }

    fn reset (&mut self) {
        if !self.fixed() {
            self.value = F::ZERO;
        }
    }
}
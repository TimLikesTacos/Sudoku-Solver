use crate::constants::MAX_NUM;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flag<T>
{
    flags: T,
    count: u8,
}

pub trait FlagTrait
where Self: Sized
{
    type IntForFlag;
    /// Contains all 1's in the size from 1 to MAX_NUM. Used for bitwise negation.
    const NEG: Self::IntForFlag;

    fn get_flags(&self) -> Self::IntForFlag;
    fn add_flag (&self, v:Self) -> Self;
    fn add_num (&mut self, v: Self::IntForFlag) -> Self;
    fn remove_flag (&mut self, v:Self) -> Self;
    fn remove_num (&mut self, v: Self::IntForFlag) -> Self;
    fn clear (&mut self);
    fn new (v: Self::IntForFlag) -> Self;
    fn count (&self) -> u8;
    fn bits () -> u8;
    fn merge (slice: &[Self] ) -> Self;
    fn is_single(&self) -> bool;
    fn set_initial(present_values: Self) -> Self;
}

/**
todo: macro-ize
**/

impl FlagTrait for Flag<u16> {

    type IntForFlag = u16;
    const NEG: u16 = 0b111111111;

    fn get_flags (&self) -> Self::IntForFlag {self.flags}

    fn add_flag (&self, v: Flag<u16>) -> Self {
        let mut f = self.flags;
        f |= v.flags;
        let mut count: u8=0;
        let mut n = f;
        while n!= 0 {
            n = n & (n-1);
            count += 1;
        }

        Flag {
            flags: f,
            count: count,
        }
    }

    fn add_num (&mut self, v: u16) -> Self {
        let add = Flag::from(v as usize).flags;
        if add & self.flags != add {
            self.count += 1;
            self.flags |= add;
        }
        self.clone()
    }

    fn remove_flag (&mut self, v: Flag<u16>) -> Self{
        self.flags &= !v.flags;
        let mut n = self.flags;
        let mut count: u8=0;
        while n != 0 {
            n = n & (n-1);
            count += 1;
        }
        self.count = count;
        self.clone()
    }

    fn remove_num (&mut self, v: u16) -> Self {
        let sub = Flag::from(v as usize).flags;
        if self.flags & sub == sub {
            self.flags ^= sub;
            self.count -= 1;
        }
        self.clone()
    }

    fn clear (&mut self) {
        self.flags = 0;
        self.count = 0;
    }

    fn new (v: u16) -> Self {
        Flag {
            flags: v,
            count: v.count_ones() as u8
        }
    }

    fn count (&self) -> u8 {self.count}

    fn bits () -> u8 {
        std::mem::size_of::<u16>() as u8 * 8
    }

    fn merge (slice: &[Self]) -> Self {
        // Start with 0, bitwise OR each flags value in slice
        let flags: u16 = slice.iter().fold(0, |acc, x| acc | x.flags );
        Flag::new(flags)
    }

    fn is_single (&self) -> bool {
        if self.count == 1 {
            true
        } else {
            false
        }
    }

    fn set_initial(present_values: Self) -> Self {
        Self::new(present_values.flags ^ Self::NEG)
    }
}


impl Default for Flag<u16>
{
    fn default () -> Flag<u16> {
        Flag::new(0)
    }
}

/**
todo: macro-ize this to allow for different primitive integers
**/
impl From<usize> for Flag<u16>
 {
    fn from (item: usize) -> Flag<u16> {
        if item == 0 {
            Flag {
                flags: 0,
                count: 0,
            }
        } else {
            Flag {
                flags: 1 << (item - 1),
                count: 1,
            }
        }
    }
}

impl From<Flag<u16>> for usize {
    fn from (item: Flag<u16>) -> usize {
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


#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn set_test() {
        let  t1: Flag<u16> = Flag::new(0);
        let mut t1 = t1.add_flag(2.into());

        assert_eq!(usize::from(t1), 2);
        t1 = t1.add_num(8);
        dbg!(&t1);
        assert_eq!(t1.flags, 0b10000010);
    }

    #[test]
    fn remove_test () {
        let mut t1 = Flag::from(8);
        t1 = t1.add_flag(Flag::from(4));
        assert_eq!(t1.count, 2);
        assert_eq!(t1.flags, 0b10001000);
        t1.remove_flag(Flag::from(8));
        assert_eq!(t1.flags, 0b1000);
        assert_eq!(t1.count, 1);
        t1.remove_num(1);
        assert_eq!(t1.count, 1);
        assert_eq!(t1.flags, 0b1000);
        t1.remove_flag(Flag::from(4));
        assert!(t1.count == 0);
        assert!(t1.flags == 0);

        let mut t1 = Flag::from(1);
        let mut t2 = Flag::from(4);
        t1 = t1.add_flag(t2);
        assert_eq!(t1.flags, 0b1001);
        assert_eq!(t1.count, 2);
        let mut t3 = Flag::from(9);
        assert_eq!(t3.flags, 0b100000000);
        t3 = t3.add_flag(t1);
        assert_eq!(t3.flags, 0b100001001);
        assert_eq!(t3.count, 3);
        t3 = t3.remove_flag(t2);
        assert_eq!(t3.flags, 0b100000001);
        assert_eq!(t3.count, 2);
        t3 = t3.add_flag(t1);
        assert_eq!(t3.flags, 0b100001001);
        assert_eq!(t3.count(), 3);
    }

    #[test]
    fn clear_test () {
        let mut t1 = Flag::new(0b1100);
        assert_eq!(t1.count(), 2);
        t1.clear();
        assert_eq!(t1, 0.into());
        assert_eq!(t1.count(), 0);
    }

    #[test]
    fn from_into() {
        let mut t1:Flag<u16> = Flag::from(4);
        assert_eq!(t1.flags, 0b1000);
        assert_eq!(t1.add_flag(Flag::from(8)).flags, 0b10001000);

    }

    #[test]
    fn merge_test() {
        let flags = [Flag::new(0b10000), Flag::new(0b00000), Flag::new(0b00100), Flag::new(0b10010), Flag::new(0b00001)];
        let res = Flag::merge (&flags[2..]);
        assert_eq!(res.flags, 0b10111);
        assert_eq!(res.count, 4);
        let res = Flag::merge (&flags [0..=2]);
        assert_eq!(res.flags, 0b10100);
        assert_eq!(res.count, 2);
    }

    #[test]
    fn flag16neg () {
        let res: u16 = Flag::NEG;
        assert_eq!(res, 0b111111111);
    }

    #[test]
    fn get_initial_test() {
        let a = Flag::new(0b0101);
        let res = Flag::set_initial(a);
        assert_eq!(res.flags, 0b111111010);

    }
}


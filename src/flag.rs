

// The Cell struct contains the number, boolean if it is fixed, and functions to incremement
type Element = u16;

#[derive(Clone, PartialEq, Debug)]
pub struct CellP {
    num: Element,
    fixed: bool,
    pencil: u16,
    pcount: Element,
}

impl Default for CellP {
    fn default() -> Self {
        CellP {
            num: 0,
            fixed: false,
            pencil: 0,
            pcount: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flag<T> {
    flags: T,
    count: u8,
}

pub trait  FlagTrait {
    fn add_flag (&self, v:Self) -> Self;
    fn add_num (&mut self, v: usize) -> Self;
    fn remove_flag (&mut self, v:Self) -> Self;
    fn remove_num (&mut self, v: usize) -> Self;
    fn clear (&mut self);
    fn new (v: usize) -> Self;
    fn count (&self) -> u8;
    fn bits () -> u8;
}

impl FlagTrait for Flag<u16> {


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

    fn add_num (&mut self, v: usize) -> Self {
        let add = Flag::new(v).flags;
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

    fn remove_num (&mut self, v: usize) -> Self {
        let sub = Flag::from(v).flags;
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

    fn new (v: usize) -> Self {
        if v == 0 {
            Flag {
                flags: 0,
                count: 0,
            }
        } else {
            Flag {
                flags: 1 << (v - 1),
                count: 1,
            }
        }
    }

    fn count (&self) -> u8 {self.count}

    fn bits () -> u8 {
        std::mem::size_of::<u16>() as u8 * 8
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
        Flag {
            flags: item as u16,
            count: item.count_ones() as u8,
        }
    }
}

impl From<Flag<u16>> for usize {
    fn from (item: Flag<u16>) -> usize {
        let mut v: u16 = item.flags;
        let mut num: u16 = 0;
        let mut place = 1;
        while v > 0 {
            if v & 1 > 0 {
                num += place;
            }
            v >>= 1;
            place += 1;
        }
        num as usize
    }
}


#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn set_test() {
        let  t1: Flag<u16> = Flag::new(0);
        dbg!(&t1);
        let mut t1 = t1.add_flag(2.into());
        dbg!(&t1);
        dbg!(usize::from(t1));
        assert_eq!(usize::from(t1), 2);
        t1 = t1.add_num(8);
        dbg!(&t1);
        assert_eq!(usize::from(t1), 10);
    }

    #[test]
    fn remove_test () {
        let mut t1 = Flag::from(0b10000000);
        t1 = t1.add_flag(Flag::from(0b1000));
        assert_eq!(t1.count, 2);
        assert_eq!(t1.flags, 0b10001000);
        t1.remove_flag(Flag::from(10000000));
        assert_eq!(t1.flags, 0b1000);
        assert_eq!(t1.count, 1);
        t1.remove_num(1);
        assert_eq!(t1.count, 1);
        assert_eq!(t1.flags, 0b1000);
        t1.remove_flag(Flag::from(0b1000));
        assert!(t1.count == 0);
        assert!(t1.flags == 0);

        let mut t1 = Flag::new(1);
        let mut t2 = Flag::new(4);
        t1 = t1.add_flag(t2);
        assert_eq!(t1.flags, 0b1001);
        assert_eq!(t1.count, 2);
        let mut t3 = Flag::from(0b100000000);
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
        assert_eq!(t1.count(), 1);
        t1.clear();
        assert_eq!(t1, 0.into());
        assert_eq!(t1.count(), 0);
    }

    #[test]
    fn from_into() {
        let mut t1:Flag<u16> = Flag::from(0b1000);
        assert_eq!(t1.flags, 0b1000);
        assert_eq!(t1.add_flag(Flag::from(0b10000000)).flags, 0b10001000);

    }
}


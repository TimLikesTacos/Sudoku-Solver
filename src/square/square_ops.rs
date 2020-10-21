use crate::square::*;
use crate::sq_element::flag_limits::*;
use std::ops::Add;
use crate::sq_element::{ValueTr, Flag};

// Combine many required traits into one.


// impl <V: ValueTr> IncReset <V> for SimpleSquare<V> {
//     fn inc(&mut self) -> bool {
//         if self.fixed || self.value == V::VMAX {
//             false
//         } else {
//             let old = self.value;
//             self.value = old + V::VONE;
//             true
//         }
//     }
//     fn reset (&mut self) {
//         if !self.fixed {
//             self.value = V::ZERO;
//         }
//     }
// }
//
// impl <F: Flag> IncReset <F> for FlagSquare<F> {
//     fn inc(&mut self) -> bool {
//         self.value.inc()
//     }
//     fn reset(&mut self) {
//         self.value.reset()
//     }
// }



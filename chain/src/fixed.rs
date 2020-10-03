// use crate::error::Error::{
//     FixedParseAbnormalChar, FixedParseAmountFormat, FixedParseDoubleDot, FixedParseOverflow,
// };
// use crate::{Error, Result};
// use alloc::string::String;
// use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
//
// #[derive(Clone, Copy)]
// pub struct Fixed {
//     value: i64,
//     decimal: isize,
// }
//
// impl Fixed {
//     fn new(o_amount: String, decimal: isize) -> Result<Fixed> {
//         let amount: Vec<char> = o_amount.chars().collect();
//
//         if amount.len() == 0 || amount[0] == '.' {
//             return Err(FixedParseAmountFormat());
//         }
//         if amount[0] == '-' {
//             match Fixed::new(
//                 (&amount[1..].iter().clone().collect::<String>())
//                     .parse()
//                     .unwrap(),
//                 decimal,
//             ) {
//                 Err(e) => return Err(e),
//                 Ok(fpn) => {
//                     return (&fpn).neg();
//                 }
//             }
//         }
//         parse_positive_fixed(o_amount, decimal)
//     }
//
//     fn marshal(&self) -> String {
//         let mut b1 = vec![0u8, 8];
//         b1.write_u64::<LittleEndian>(self.value as u64);
//         // LittleEndian::write_u64(b1, self.value as u64);
//         let mut b2 = vec![0u8, 4];
//         b2.write_u32::<LittleEndian>(self.decimal as u32);
//         // LittleEndian::write_u32(b2, self.decimal as u32);
//         let s1 = String::from_utf8(b1).unwrap();
//         let s2 = String::from_utf8(b2).unwrap();
//         s1 + &s2
//     }
//
//     pub fn is_zero(&self) -> bool {
//         self.value == 0
//     }
//
//     pub fn is_positive(&self) -> bool {
//         self.value > 0
//     }
//
//     pub fn is_negative(&self) -> bool {
//         self.value < 0
//     }
//
//     pub fn neg(&self) -> Result<Fixed> {
//         if multiply_overflow(self.value, -1) {
//             Err(FixedParseOverflow())
//         } else {
//             Ok(Fixed {
//                 value: -self.value,
//                 decimal: self.decimal.clone(),
//             })
//         }
//     }
//
//     // ChangeDecimal change decimal to give decimal, without changing its real value
//     pub fn change_decimal(&self, target: isize) -> Result<Fixed> {
//         let mut value = self.value;
//         let mut decimal = self.decimal as isize;
//         while target > decimal {
//             decimal = decimal + 1;
//             if multiply_overflow(value, 10) {
//                 return Err(FixedParseOverflow());
//             }
//             value *= 10;
//         }
//         while target < decimal {
//             decimal = decimal - 1;
//             value /= 10;
//         }
//         Ok(Fixed { value, decimal })
//     }
//
//     // ShrinkDecimal remove trailing 0s
//     pub fn shrink_decimal(&self) -> Result<Fixed> {
//         let mut value = self.value;
//         let mut decimal = self.decimal;
//         while value % 10 == 0 && decimal > 0 {
//             value /= 10;
//             decimal = decimal - 1;
//         }
//         Ok(Fixed { value, decimal })
//     }
//
//     pub fn equals(&self, other: &Fixed) -> bool {
//         match unify_decimal(self, other) {
//             Ok((fpn_new, other_new)) => fpn_new.value == other_new.value,
//             _ => false,
//         }
//     }
// }
//
// pub fn multiply_overflow(a: i64, b: i64) -> bool {
//     let x = a * b;
//     a != 0 && x / a != b
// }
//
// pub fn parse_positive_fixed(amount: String, decimal: isize) -> Result<Fixed> {
//     let mut fpn = Fixed {
//         value: 0,
//         decimal: 0,
//     };
//     let mut decimal_start = false;
//     let amount: Vec<char> = amount.chars().collect();
//     for i in 0..amount.len() {
//         if amount[i] == '.' {
//             if decimal_start {
//                 return Err(FixedParseDoubleDot());
//             }
//             decimal_start = true;
//         } else {
//             match amount[i].to_digit(10) {
//                 Some(n) => {
//                     if 0 <= n && n <= 9 {
//                         if multiply_overflow(fpn.value, 10) {
//                             return Err(FixedParseOverflow());
//                         }
//                         fpn.value = fpn.value * 10 + n as i64;
//
//                         if fpn.value < 0 {
//                             return Err(FixedParseOverflow());
//                         }
//                         if decimal_start {
//                             fpn.decimal = fpn.decimal + 1;
//                             if decimal > 0 && fpn.decimal >= decimal {
//                                 break;
//                             }
//                         }
//                     } else {
//                         return Err(FixedParseAbnormalChar());
//                     }
//                 }
//                 None => return Err(FixedParseAbnormalChar()),
//             }
//         }
//     }
//     Ok(fpn)
// }
//
// pub fn unify_decimal(a: &Fixed, b: &Fixed) -> Result<(Fixed, Fixed)> {
//     if a.decimal < b.decimal {
//         match a.change_decimal(b.decimal) {
//             Ok(changed) => Ok((changed, b.clone())),
//             Err(e) => Err(e),
//         }
//     } else {
//         match b.change_decimal(a.decimal) {
//             Err(e) => Err(e),
//             Ok(changed) => Ok((a.clone(), changed)),
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     async fn fixed_marshal() {
//         let f = Fixed::new(String::from("-323.49494"), 12)?;
//         assert!(f.Value, -323494940000000 as i64)
//     }
// }

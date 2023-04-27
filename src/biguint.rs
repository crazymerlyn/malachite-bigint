use derive_more::{Binary, From, LowerHex, Octal, UpperHex};
use malachite::{
    num::{
        conversion::traits::{Digits, PowerOf2Digits, ToStringBase},
        logic::traits::{BitAccess, CountOnes, SignificantBits},
    },
    Natural,
};
use num_integer::Roots;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, Pow, Unsigned, Zero};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

use crate::ParseBigIntError;

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Debug,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    From,
)]
#[from(forward)]
pub struct BigUint(Natural);

forward_binary_self!(BigUint, Add, add);
forward_binary_self!(BigUint, Sub, sub);
forward_binary_self!(BigUint, Mul, mul);
forward_binary_self!(BigUint, Div, div);
forward_binary_self!(BigUint, Rem, rem);
// // forward_binary_self!(BigUint, Pow, pow, malachite::num::arithmetic::traits::Pow::pow);
forward_binary_self!(BigUint, BitAnd, bitand);
forward_binary_self!(BigUint, BitOr, bitor);
forward_binary_self!(BigUint, BitXor, bitxor);

forward_assign_self!(BigUint, AddAssign, add_assign);
forward_assign_self!(BigUint, SubAssign, sub_assign);
forward_assign_self!(BigUint, MulAssign, mul_assign);
forward_assign_self!(BigUint, DivAssign, div_assign);
forward_assign_self!(BigUint, RemAssign, rem_assign);
forward_assign_self!(BigUint, BitAndAssign, bitand_assign);
forward_assign_self!(BigUint, BitOrAssign, bitor_assign);
forward_assign_self!(BigUint, BitXorAssign, bitxor_assign);

apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Add, add});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Sub, sub});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Mul, mul});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Div, div});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Rem, rem});

apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Add, add});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Sub, sub});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Mul, mul});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Div, div});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Rem, rem});

apply_to_primitives!(forward_binary_right_primitive{BigUint, _, Shl, shl});
apply_to_primitives!(forward_binary_right_primitive{BigUint, _, Shr, shr});

apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, AddAssign, add_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, SubAssign, sub_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, MulAssign, mul_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, DivAssign, div_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, RemAssign, rem_assign});

apply_to_primitives!(forward_assign_primitive{BigUint, _, ShlAssign, shl_assign});
apply_to_primitives!(forward_assign_primitive{BigUint, _, ShrAssign, shr_assign});

apply_to_unsigneds!(forward_pow_primitive{BigUint, _});
// TODO: pow self

impl CheckedAdd for BigUint {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(self.add(v))
    }
}

impl CheckedSub for BigUint {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        match self.cmp(v) {
            Less => None,
            Equal => Some(Self::zero()),
            Greater => Some(self.sub(v)),
        }
    }
}

impl CheckedMul for BigUint {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(self.mul(v))
    }
}

impl CheckedDiv for BigUint {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        (!v.is_zero()).then(|| self.div(v))
    }
}

impl Zero for BigUint {
    fn zero() -> Self {
        Self(<Natural as malachite::num::basic::traits::Zero>::ZERO)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for BigUint {
    fn one() -> Self {
        Self(<Natural as malachite::num::basic::traits::One>::ONE)
    }
}

impl Unsigned for BigUint {}

impl Num for BigUint {
    type FromStrRadixErr = ParseBigIntError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl num_integer::Integer for BigUint {
    fn div_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn gcd(&self, other: &Self) -> Self {
        todo!()
    }

    fn lcm(&self, other: &Self) -> Self {
        todo!()
    }

    fn divides(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_even(&self) -> bool {
        todo!()
    }

    fn is_odd(&self) -> bool {
        todo!()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        todo!()
    }
}

impl Roots for BigUint {
    fn nth_root(&self, n: u32) -> Self {
        todo!()
    }
}

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}

impl FromStr for BigUint {
    type Err = ParseBigIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10)
    }
}

impl BigUint {
    pub fn new(digits: Vec<u32>) -> Self {
        todo!()
    }

    #[inline]
    pub fn from_slice(slice: &[u32]) -> Self {
        let mut uint = BigUint::zero();
        uint.assign_from_slice(slice);
        uint
    }

    pub fn assign_from_slice(&mut self, slice: &[u32]) {
        todo!()
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        // SAFETY: &[u8] cannot have any digit greater than 2^8
        Self(unsafe {
            Natural::from_power_of_2_digits_desc(8, bytes.iter().cloned()).unwrap_unchecked()
        })
    }

    #[inline]
    pub fn from_bytes_le(bytes: &[u8]) -> Self {
        // SAFETY: &[u8] cannot have any digit greater than 2^8
        Self(unsafe {
            Natural::from_power_of_2_digits_asc(8, bytes.iter().cloned()).unwrap_unchecked()
        })
    }

    #[inline]
    pub fn parse_bytes(bytes: &[u8], radix: u32) -> Option<BigUint> {
        let s = std::str::from_utf8(bytes).ok()?;
        Self::from_str_radix(s, radix).ok()
    }

    pub fn from_radix_be(bytes: &[u8], radix: u32) -> Option<Self> {
        todo!()
    }

    pub fn from_radix_le(bytes: &[u8], radix: u32) -> Option<Self> {
        todo!()
    }

    #[inline]
    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_power_of_2_digits_desc(8)
    }

    #[inline]
    pub fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_power_of_2_digits_asc(8)
    }

    #[inline]
    pub fn to_u32_digits(&self) -> Vec<u32> {
        self.0.to_power_of_2_digits_asc(32)
    }

    #[inline]
    pub fn to_u64_digits(&self) -> Vec<u64> {
        self.0.to_limbs_asc()
    }

    pub fn iter_u32_digits(&self) {
        todo!()
    }

    pub fn iter_u64_digits(&self) {
        todo!()
    }

    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        self.0.to_string_base(radix as u8)
    }

    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        debug_assert!(radix <= 256);
        if radix == 256 {
            self.to_bytes_be()
        } else {
            self.0.to_digits_desc(&(radix as u8))
        }
    }

    #[inline]
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        debug_assert!(radix <= 256);
        if radix == 256 {
            self.to_bytes_le()
        } else {
            self.0.to_digits_asc(&(radix as u8))
        }
    }

    #[inline]
    pub fn bits(&self) -> u64 {
        self.0.significant_bits()
    }

    pub fn pow(&self, exponent: u32) -> Self {
        todo!()
    }

    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        todo!()
    }

    #[inline]
    pub fn cbrt(&self) -> Self {
        Roots::cbrt(self)
    }

    #[inline]
    pub fn nth_root(&self, n: u32) -> Self {
        Roots::nth_root(self, n)
    }

    #[inline]
    pub fn trailing_zeros(&self) -> Option<u64> {
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn trailing_ones(&self) -> u64 {
        todo!()
    }

    #[inline]
    pub fn count_ones(&self) -> u64 {
        self.0.count_ones()
    }

    #[inline]
    pub fn bit(&self, bit: u64) -> bool {
        self.0.get_bit(bit)
    }

    #[inline]
    pub fn set_bit(&mut self, bit: u64, value: bool) {
        if value {
            self.0.set_bit(bit)
        } else {
            self.0.clear_bit(bit)
        }
    }
}

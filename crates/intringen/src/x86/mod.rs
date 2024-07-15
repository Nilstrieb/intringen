#![allow(non_camel_case_types, non_snake_case)]

mod generated;

pub use generated::soft_arch;

pub trait Core {
    type u8: Copy;
    type u16: Copy;
    type u32: Copy;
    type u64: Copy;

    type __m128i: Copy;

    fn get_lane___m128i_u8(&mut self, value: Self::__m128i, idx: u64) -> Self::u8;
    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16;
    fn get_lane___m128i_u32(&mut self, value: Self::__m128i, idx: u64) -> Self::u32;
    fn get_lane___m128i_u64(&mut self, value: Self::__m128i, idx: u64) -> Self::u64;

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8);
    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16);
    fn set_lane___m128i_u32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u32);
    fn set_lane___m128i_u64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u64);

    fn saturate8(&mut self, elem: Self::u16) -> Self::u8;
    fn saturate_u8(&mut self, elem: Self::u16) -> Self::u8;
    fn saturate16(&mut self, elem: Self::u32) -> Self::u16;
    fn saturate_u16(&mut self, elem: Self::u32) -> Self::u16;
    fn add_64(&mut self, lhs: Self::u64, rhs: Self::u64) -> Self::u64;

    fn abs_u8(&mut self, x: Self::u8) -> Self::u8;
    fn abs_u16(&mut self, x: Self::u16) -> Self::u16;
    fn abs_u32(&mut self, x: Self::u32) -> Self::u32;
    fn abs_u64(&mut self, x: Self::u64) -> Self::u64;
}

pub struct ValueCore;

impl Core for ValueCore {
    type u8 = u8;
    type u16 = u16;
    type u32 = u32;
    type u64 = u64;

    type __m128i = [u8; 16];

    ////// GET LANE
    fn get_lane___m128i_u8(&mut self, value: Self::__m128i, idx: u64) -> Self::u8 {
        value[idx as usize]
    }

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16 {
        let mut acc = 0;
        for i in 0..2 {
            let v = value[(idx * 2 + i) as usize];
            acc |= (v as u16) << (8 * i);
        }

        acc
    }

    fn get_lane___m128i_u32(&mut self, value: Self::__m128i, idx: u64) -> Self::u32 {
        let mut acc = 0;
        for i in 0..4 {
            let v = value[(idx * 4 + i) as usize];
            acc |= (v as u32) << (8 * i);
        }

        acc
    }

    fn get_lane___m128i_u64(&mut self, value: Self::__m128i, idx: u64) -> Self::u64 {
        let mut acc = 0;
        for i in 0..8 {
            let v = value[(idx * 8 + i) as usize];
            acc |= (v as u64) << (8 * i);
        }

        acc
    }

    ////// SET LANE

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8) {
        place[idx as usize] = value;
    }

    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16) {
        for i in 0..2 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 2 + i) as usize] = value;
        }
    }

    fn set_lane___m128i_u32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u32) {
        for i in 0..4 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 4 + i) as usize] = value;
        }
    }

    fn set_lane___m128i_u64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u64) {
        for i in 0..8 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 8 + i) as usize] = value;
        }
    }

    ////// HELPERS

    fn saturate8(&mut self, elem: Self::u16) -> Self::u8 {
        let clamp = (elem as i16).clamp(i8::MIN as i16, i8::MAX as i16);
        clamp as i8 as u8
    }

    fn saturate_u8(&mut self, elem: Self::u16) -> Self::u8 {
        let clamp = (elem as i16).clamp(0, u8::MAX as i16);
        clamp as u8
    }

    fn saturate16(&mut self, elem: Self::u32) -> Self::u16 {
        let clamp = (elem as i32).clamp(i16::MIN as i32, i16::MAX as i32);
        clamp as i16 as u16
    }
    fn saturate_u16(&mut self, elem: Self::u32) -> Self::u16 {
        let clamp = (elem as i32).clamp(0, u16::MAX as i32);
        clamp as u16
    }
    fn add_64(&mut self, lhs: Self::u64, rhs: Self::u64) -> Self::u64 {
        lhs.wrapping_add(rhs)
    }

    fn abs_u8(&mut self, x: Self::u8) -> Self::u8 {
        (x as i8).abs() as u8
    }

    fn abs_u16(&mut self, x: Self::u16) -> Self::u16 {
        (x as i16).abs() as u16
    }

    fn abs_u32(&mut self, x: Self::u32) -> Self::u32 {
        (x as i32).abs() as u32
    }

    fn abs_u64(&mut self, x: Self::u64) -> Self::u64 {
        (x as i64).abs() as u64
    }
}

mod soft_arch_types {
    pub type __m128i = [u8; 16];
}

#[cfg(all(test, target_arch = "x86_64"))]
mod compare_test_helper {
    macro_rules! hard_soft_same_128 {
        ($($stmt:tt)*) => {
            let soft = {
                use crate::x86::soft_arch::*;
                $($stmt)*
            };
            let hard = unsafe {
                std::mem::transmute::<_, [u8; 16]>({
                    use core::arch::x86_64::*;
                    $($stmt)*
                })
            };
            assert_eq!(soft, hard);
        };
    }
    pub(super) use hard_soft_same_128;
}

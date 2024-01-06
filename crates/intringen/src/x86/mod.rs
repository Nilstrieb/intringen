#![allow(non_camel_case_types, non_snake_case)]

mod generated;

pub use generated::soft_arch;

pub trait Core {
    type u8: Copy;
    type u16: Copy;
    type u32: Copy;
    type u64: Copy;

    type i8: Copy;
    type i16: Copy;
    type i32: Copy;
    type i64: Copy;

    type __m128i: Copy;

    fn cast_sign_i8_u8(&mut self, value: Self::i8) -> Self::u8;
    fn cast_sign_i16_u16(&mut self, value: Self::i16) -> Self::u16;
    fn cast_sign_i32_u32(&mut self, value: Self::i32) -> Self::u32;
    fn cast_sign_i64_u64(&mut self, value: Self::i64) -> Self::u64;

    fn get_lane___m128i_u8(&mut self, value: Self::__m128i, idx: u64) -> Self::u8;
    fn get_lane___m128i_i8(&mut self, value: Self::__m128i, idx: u64) -> Self::i8;
    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16;
    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> Self::i16;
    fn get_lane___m128i_u32(&mut self, value: Self::__m128i, idx: u64) -> Self::u32;
    fn get_lane___m128i_i32(&mut self, value: Self::__m128i, idx: u64) -> Self::i32;
    fn get_lane___m128i_u64(&mut self, value: Self::__m128i, idx: u64) -> Self::u64;
    fn get_lane___m128i_i64(&mut self, value: Self::__m128i, idx: u64) -> Self::i64;

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8);
    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i8);
    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16);
    fn set_lane___m128i_i16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i16);
    fn set_lane___m128i_u32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u32);
    fn set_lane___m128i_i32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i32);
    fn set_lane___m128i_u64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u64);
    fn set_lane___m128i_i64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i64);

    fn saturate8(&mut self, elem: Self::i16) -> Self::i8;
    fn saturate_u8(&mut self, elem: Self::i16) -> Self::u8;
    fn saturate16(&mut self, elem: Self::i32) -> Self::i16;
    fn saturate_u16(&mut self, elem: Self::i32) -> Self::u16;
    fn abs_i8(&mut self, elem: Self::i8) -> Self::u8;
    fn abs_i16(&mut self, elem: Self::i16) -> Self::u16;
}

pub struct ValueCore;

impl Core for ValueCore {
    type u8 = u8;
    type u16 = u16;
    type u32 = u32;
    type u64 = u64;

    type i8 = i8;
    type i16 = i16;
    type i32 = i32;
    type i64 = i64;

    type __m128i = [u8; 16];

    ////// CAST

    fn cast_sign_i8_u8(&mut self, value: Self::i8) -> Self::u8 {
        value as _
    }

    fn cast_sign_i16_u16(&mut self, value: Self::i16) -> Self::u16 {
        value as _
    }

    fn cast_sign_i32_u32(&mut self, value: Self::i32) -> Self::u32 {
        value as _
    }

    fn cast_sign_i64_u64(&mut self, value: Self::i64) -> Self::u64 {
        value as _
    }

    ////// GET LANE

    fn get_lane___m128i_u8(&mut self, value: Self::__m128i, idx: u64) -> Self::u8 {
        value[idx as usize]
    }

    fn get_lane___m128i_i8(&mut self, value: Self::__m128i, idx: u64) -> Self::i8 {
        self.get_lane___m128i_u8(value, idx) as i8
    }

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16 {
        let mut acc = 0;
        for i in 0..2 {
            let v = value[(idx * 2 + i) as usize];
            acc |= (v as u16) << (8 * i);
        }

        acc
    }

    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> Self::i16 {
        self.get_lane___m128i_u16(value, idx) as i16
    }

    fn get_lane___m128i_u32(&mut self, value: Self::__m128i, idx: u64) -> Self::u32 {
        let mut acc = 0;
        for i in 0..4 {
            let v = value[(idx * 4 + i) as usize];
            acc |= (v as u32) << (8 * i);
        }

        acc
    }

    fn get_lane___m128i_i32(&mut self, value: Self::__m128i, idx: u64) -> Self::i32 {
        self.get_lane___m128i_u32(value, idx) as i32
    }

    fn get_lane___m128i_u64(&mut self, value: Self::__m128i, idx: u64) -> Self::u64 {
        let mut acc = 0;
        for i in 0..8 {
            let v = value[(idx * 8 + i) as usize];
            acc |= (v as u64) << (8 * i);
        }

        acc
    }

    fn get_lane___m128i_i64(&mut self, value: Self::__m128i, idx: u64) -> Self::i64 {
        self.get_lane___m128i_u64(value, idx) as i64
    }

    ////// SET LANE

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8) {
        place[idx as usize] = value;
    }

    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i8) {
        self.set_lane___m128i_u8(place, idx, value as u8);
    }

    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16) {
        for i in 0..2 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 2 + i) as usize] = value;
        }
    }

    fn set_lane___m128i_i16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i16) {
        self.set_lane___m128i_u16(place, idx, value as u16);
    }

    fn set_lane___m128i_u32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u32) {
        for i in 0..4 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 4 + i) as usize] = value;
        }
    }

    fn set_lane___m128i_i32(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i32) {
        self.set_lane___m128i_u32(place, idx, value as u32);
    }

    fn set_lane___m128i_u64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u64) {
        for i in 0..8 {
            let value = ((value >> 8 * i) & 0xFF) as u8;
            place[(idx * 8 + i) as usize] = value;
        }
    }

    fn set_lane___m128i_i64(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i64) {
        self.set_lane___m128i_u32(place, idx, value as u32);
    }

    ////// HELPERS

    fn saturate8(&mut self, elem: Self::i16) -> Self::i8 {
        let clamp = elem.clamp(i8::MIN as i16, i8::MAX as i16);
        clamp as i8
    }

    fn saturate_u8(&mut self, elem: Self::i16) -> Self::u8 {
        let clamp = elem.clamp(0, u8::MAX as i16);
        clamp as u8
    }

    fn saturate16(&mut self, elem: Self::i32) -> Self::i16 {
        let clamp = elem.clamp(i16::MIN as i32, i16::MAX as i32);
        clamp as i16
    }
    fn saturate_u16(&mut self, elem: Self::i32) -> Self::u16 {
        let clamp = elem.clamp(0, u16::MAX as i32);
        clamp as u16
    }
    fn abs_i8(&mut self, elem: Self::i8) -> Self::u8 {
        elem.abs() as u8
    }
    fn abs_i16(&mut self, elem: Self::i16) -> Self::u16 {
        elem.abs() as u16
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

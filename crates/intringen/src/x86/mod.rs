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

    fn cast_sign_i16_u16(&mut self, value: Self::i16) -> Self::u16;

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16;
    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> Self::i16;

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8);
    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i8);
    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16);

    fn saturate_u8(&mut self, elem: Self::i16) -> Self::u8;
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

    fn cast_sign_i16_u16(&mut self, value: Self::i16) -> Self::u16 {
        value as _
    }

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> Self::u16 {
        let first = value[(idx * 2 + 1) as usize];
        let second = value[(idx * 2) as usize];

        ((first as u16) << 8) | (second as u16)
    }

    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> Self::i16 {
        let first = value[(idx * 2 + 1) as usize];
        let second = value[(idx * 2) as usize];

        ((((first as u16) << 8) as u16) | (second as u16)) as i16
    }

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u8) {
        place[idx as usize] = value;
    }

    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::i8) {
        place[idx as usize] = value as u8;
    }

    fn set_lane___m128i_u16(&mut self, place: &mut Self::__m128i, idx: u64, value: Self::u16) {
        let first = (value & 0xFF) as u8;
        let second = (value >> 8) as u8;
        place[(idx * 2) as usize] = first;
        place[(idx * 2 + 1) as usize] = second;
    }

    fn saturate_u8(&mut self, elem: Self::i16) -> Self::u8 {
        let clamp = elem.clamp(0, u8::MAX as i16);
        clamp as u8
    }
}

mod soft_arch_types {
    pub type __m128i = [u8; 16];
}

#[cfg(test)]
mod tests;
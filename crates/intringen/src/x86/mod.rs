#![allow(non_camel_case_types, non_snake_case)]

mod generated;

pub trait Core {
    type __m128i: Copy;

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> u16;
    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> i16;

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: u8);
    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: i8);

    fn saturate_u8(&mut self, elem: i16) -> u8;
}

pub struct ValueCore;

impl Core for ValueCore {
    type __m128i = [u8; 16];

    fn get_lane___m128i_u16(&mut self, value: Self::__m128i, idx: u64) -> u16 {
        let first = value[(idx * 2) as usize];
        let second = value[(idx * 2 + 1) as usize];
        // todo: le? be?
        ((first << 8) as u16) | (second as u16)
    }

    fn get_lane___m128i_i16(&mut self, value: Self::__m128i, idx: u64) -> i16 {
        let first = value[(idx * 2) as usize];
        let second = value[(idx * 2 + 1) as usize];
        // todo: le? be?
        (((first << 8) as u16) | (second as u16)) as i16
    }

    fn set_lane___m128i_u8(&mut self, place: &mut Self::__m128i, idx: u64, value: u8) {
        place[idx as usize] = value;
    }

    fn set_lane___m128i_i8(&mut self, place: &mut Self::__m128i, idx: u64, value: i8) {
        place[idx as usize] = value as u8;
    }

    fn saturate_u8(&mut self, elem: i16) -> u8 {
        let clamp = elem.clamp(0, u8::MAX as i16);
        clamp as u8
    }
}
pub trait Lanes<const N: usize, Elem> {}

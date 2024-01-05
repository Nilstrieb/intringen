impl<C: super::Core> Intrinsics for C {}
pub trait Intrinsics: super::Core {
    fn _mm_setr_epi16(&mut self, dst: &mut Self::__m128i, e7: Self::i16, e6: Self::i16, e5: Self::i16, e4: Self::i16, e3: Self::i16, e2: Self::i16, e1: Self::i16, e0: Self::i16) {
        let __tmp = self.cast_sign_i16_u16(e7);
        self.set_lane___m128i_u16(dst, 0, __tmp);
        let __tmp = self.cast_sign_i16_u16(e6);
        self.set_lane___m128i_u16(dst, 1, __tmp);
        let __tmp = self.cast_sign_i16_u16(e5);
        self.set_lane___m128i_u16(dst, 2, __tmp);
        let __tmp = self.cast_sign_i16_u16(e4);
        self.set_lane___m128i_u16(dst, 3, __tmp);
        let __tmp = self.cast_sign_i16_u16(e3);
        self.set_lane___m128i_u16(dst, 4, __tmp);
        let __tmp = self.cast_sign_i16_u16(e2);
        self.set_lane___m128i_u16(dst, 5, __tmp);
        let __tmp = self.cast_sign_i16_u16(e1);
        self.set_lane___m128i_u16(dst, 6, __tmp);
        let __tmp = self.cast_sign_i16_u16(e0);
        self.set_lane___m128i_u16(dst, 7, __tmp);
    }
    fn _mm_packus_epi16(&mut self, dst: &mut Self::__m128i, a: Self::__m128i, b: Self::__m128i) {
        let __tmp = self.get_lane___m128i_i16(a, 0);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 0, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 1);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 1, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 2);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 2, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 3);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 3, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 4);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 4, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 5);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 5, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 6);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 6, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 7);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 7, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 0);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 8, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 1);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 9, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 2);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 10, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 3);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 11, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 4);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 12, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 5);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 13, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 6);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 14, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 7);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 15, __tmp);
    }
}

pub mod soft_arch {
    pub use super::super::soft_arch_types::*;
    use super::Intrinsics;
    pub fn _mm_setr_epi16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_setr_epi16(&mut output, e7, e6, e5, e4, e3, e2, e1, e0);
        output
    }
    pub fn _mm_packus_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packus_epi16(&mut output, a, b);
        output
    }
}

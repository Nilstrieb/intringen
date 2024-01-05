impl<C: super::Core> Intrinsics for C {}
pub trait Intrinsics: super::Core {
    fn _mm_set_epi64x(&mut self, dst: &mut Self::__m128i, e1: Self::i64, e0: Self::i64) {
        let __tmp = self.cast_sign_i64_u64(e0);
        self.set_lane___m128i_u64(dst, 0u64, __tmp);
        let __tmp = self.cast_sign_i64_u64(e1);
        self.set_lane___m128i_u64(dst, 1u64, __tmp);
    }
    fn _mm_setr_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        e3: Self::i32,
        e2: Self::i32,
        e1: Self::i32,
        e0: Self::i32,
    ) {
        let __tmp = self.cast_sign_i32_u32(e3);
        self.set_lane___m128i_u32(dst, 0u64, __tmp);
        let __tmp = self.cast_sign_i32_u32(e2);
        self.set_lane___m128i_u32(dst, 1u64, __tmp);
        let __tmp = self.cast_sign_i32_u32(e1);
        self.set_lane___m128i_u32(dst, 2u64, __tmp);
        let __tmp = self.cast_sign_i32_u32(e0);
        self.set_lane___m128i_u32(dst, 3u64, __tmp);
    }
    fn _mm_setr_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        e7: Self::i16,
        e6: Self::i16,
        e5: Self::i16,
        e4: Self::i16,
        e3: Self::i16,
        e2: Self::i16,
        e1: Self::i16,
        e0: Self::i16,
    ) {
        let __tmp = self.cast_sign_i16_u16(e7);
        self.set_lane___m128i_u16(dst, 0u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e6);
        self.set_lane___m128i_u16(dst, 1u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e5);
        self.set_lane___m128i_u16(dst, 2u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e4);
        self.set_lane___m128i_u16(dst, 3u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e3);
        self.set_lane___m128i_u16(dst, 4u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e2);
        self.set_lane___m128i_u16(dst, 5u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e1);
        self.set_lane___m128i_u16(dst, 6u64, __tmp);
        let __tmp = self.cast_sign_i16_u16(e0);
        self.set_lane___m128i_u16(dst, 7u64, __tmp);
    }
    fn _mm_setr_epi8(
        &mut self,
        dst: &mut Self::__m128i,
        e15: Self::i8,
        e14: Self::i8,
        e13: Self::i8,
        e12: Self::i8,
        e11: Self::i8,
        e10: Self::i8,
        e9: Self::i8,
        e8: Self::i8,
        e7: Self::i8,
        e6: Self::i8,
        e5: Self::i8,
        e4: Self::i8,
        e3: Self::i8,
        e2: Self::i8,
        e1: Self::i8,
        e0: Self::i8,
    ) {
        let __tmp = self.cast_sign_i8_u8(e15);
        self.set_lane___m128i_u8(dst, 0u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e14);
        self.set_lane___m128i_u8(dst, 1u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e13);
        self.set_lane___m128i_u8(dst, 2u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e12);
        self.set_lane___m128i_u8(dst, 3u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e11);
        self.set_lane___m128i_u8(dst, 4u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e10);
        self.set_lane___m128i_u8(dst, 5u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e9);
        self.set_lane___m128i_u8(dst, 6u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e8);
        self.set_lane___m128i_u8(dst, 7u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e7);
        self.set_lane___m128i_u8(dst, 8u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e6);
        self.set_lane___m128i_u8(dst, 9u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e5);
        self.set_lane___m128i_u8(dst, 10u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e4);
        self.set_lane___m128i_u8(dst, 11u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e3);
        self.set_lane___m128i_u8(dst, 12u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e2);
        self.set_lane___m128i_u8(dst, 13u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e1);
        self.set_lane___m128i_u8(dst, 14u64, __tmp);
        let __tmp = self.cast_sign_i8_u8(e0);
        self.set_lane___m128i_u8(dst, 15u64, __tmp);
    }
    fn _mm_packs_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp = self.get_lane___m128i_i16(a, 0u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 0u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 1u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 1u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 2u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 2u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 3u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 3u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 4u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 4u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 5u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 5u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 6u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 6u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 7u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 7u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 0u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 8u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 1u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 9u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 2u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 10u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 3u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 11u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 4u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 12u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 5u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 13u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 6u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 14u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 7u64);
        let __tmp = self.saturate8(__tmp);
        self.set_lane___m128i_i8(dst, 15u64, __tmp);
    }
    fn _mm_packs_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp = self.get_lane___m128i_i32(a, 0u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 0u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 1u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 1u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 2u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 2u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 3u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 3u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 0u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 4u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 1u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 5u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 2u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 6u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 3u64);
        let __tmp = self.saturate16(__tmp);
        self.set_lane___m128i_i16(dst, 7u64, __tmp);
    }
    fn _mm_packus_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp = self.get_lane___m128i_i16(a, 0u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 0u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 1u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 1u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 2u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 2u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 3u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 3u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 4u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 4u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 5u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 5u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 6u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 6u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(a, 7u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 7u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 0u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 8u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 1u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 9u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 2u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 10u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 3u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 11u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 4u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 12u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 5u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 13u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 6u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 14u64, __tmp);
        let __tmp = self.get_lane___m128i_i16(b, 7u64);
        let __tmp = self.saturate_u8(__tmp);
        self.set_lane___m128i_u8(dst, 15u64, __tmp);
    }
    fn _mm_packus_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp = self.get_lane___m128i_i32(a, 0u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 0u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 1u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 1u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 2u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 2u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(a, 3u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 3u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 0u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 4u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 1u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 5u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 2u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 6u64, __tmp);
        let __tmp = self.get_lane___m128i_i32(b, 3u64);
        let __tmp = self.saturate_u16(__tmp);
        self.set_lane___m128i_u16(dst, 7u64, __tmp);
    }
}
pub mod soft_arch {
    pub use super::super::soft_arch_types::*;
    use super::Intrinsics;
    pub fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_set_epi64x(&mut output, e1, e0);
        output
    }
    pub fn _mm_setr_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_setr_epi32(&mut output, e3, e2, e1, e0);
        output
    }
    pub fn _mm_setr_epi16(
        e7: i16,
        e6: i16,
        e5: i16,
        e4: i16,
        e3: i16,
        e2: i16,
        e1: i16,
        e0: i16,
    ) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore
            ._mm_setr_epi16(&mut output, e7, e6, e5, e4, e3, e2, e1, e0);
        output
    }
    pub fn _mm_setr_epi8(
        e15: i8,
        e14: i8,
        e13: i8,
        e12: i8,
        e11: i8,
        e10: i8,
        e9: i8,
        e8: i8,
        e7: i8,
        e6: i8,
        e5: i8,
        e4: i8,
        e3: i8,
        e2: i8,
        e1: i8,
        e0: i8,
    ) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore
            ._mm_setr_epi8(
                &mut output,
                e15,
                e14,
                e13,
                e12,
                e11,
                e10,
                e9,
                e8,
                e7,
                e6,
                e5,
                e4,
                e3,
                e2,
                e1,
                e0,
            );
        output
    }
    pub fn _mm_packs_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packs_epi16(&mut output, a, b);
        output
    }
    pub fn _mm_packs_epi32(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packs_epi32(&mut output, a, b);
        output
    }
    pub fn _mm_packus_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packus_epi16(&mut output, a, b);
        output
    }
    pub fn _mm_packus_epi32(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packus_epi32(&mut output, a, b);
        output
    }
}
#[cfg(all(test, target_arch = "x86_64"))]
pub mod tests {
    use super::super::compare_test_helper::hard_soft_same_128;
    #[test]
    fn _mm_set_epi64x() {
        hard_soft_same_128! {
            { let e1 = 1041352657357235268i64; let e0 = 1955209120357942897i64;
            _mm_set_epi64x(e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi32() {
        hard_soft_same_128! {
            { let e3 = 1455669123i32; let e2 = 247864885i32; let e1 = 1390920924i32; let
            e0 = 1068333055i32; _mm_setr_epi32(e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi16() {
        hard_soft_same_128! {
            { let e7 = 16513i16; let e6 = 22878i16; let e5 = 23986i16; let e4 = 27900i16;
            let e3 = - 8343i16; let e2 = - 10648i16; let e1 = 4841i16; let e0 = 14610i16;
            _mm_setr_epi16(e7, e6, e5, e4, e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi8() {
        hard_soft_same_128! {
            { let e15 = - 99i8; let e14 = 125i8; let e13 = 118i8; let e12 = 5i8; let e11
            = 41i8; let e10 = - 40i8; let e9 = 124i8; let e8 = - 6i8; let e7 = 114i8; let
            e6 = 24i8; let e5 = - 99i8; let e4 = 65i8; let e3 = 11i8; let e2 = - 15i8;
            let e1 = 20i8; let e0 = - 107i8; _mm_setr_epi8(e15, e14, e13, e12, e11, e10,
            e9, e8, e7, e6, e5, e4, e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_packs_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(23623i16, - 22080i16, - 1436i16, - 30227i16,
            8629i16, 10922i16, - 16731i16, - 1013i16); let b = _mm_setr_epi16(- 14310i16,
            2892i16, - 28568i16, 12614i16, 20103i16, 32412i16, - 28704i16, - 27930i16);
            _mm_packs_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packs_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(4197i16, 1829i16, 9149i16, 18759i16, 30885i16, -
            3879i16, 21600i16, 24454i16); let b = _mm_setr_epi16(23524i16, 10765i16,
            32539i16, 26890i16, - 3892i16, 4386i16, 18704i16, 8253i16);
            _mm_packs_epi32(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 29217i16, 32013i16, 7448i16, 2172i16, - 14764i16,
            - 1068i16, - 25463i16, 21215i16); let b = _mm_setr_epi16(- 31392i16, -
            14015i16, - 32565i16, - 11312i16, - 4934i16, - 19283i16, - 27533i16, -
            9939i16); _mm_packus_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 9518i16, - 29742i16, 10115i16, 1617i16, 13256i16,
            - 2379i16, 19254i16, 7533i16); let b = _mm_setr_epi16(- 17891i16, 30761i16,
            2539i16, 4135i16, 26713i16, 16348i16, - 21336i16, 3595i16);
            _mm_packus_epi32(a, b) }
        }
    }
}


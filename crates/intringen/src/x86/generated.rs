impl<C: super::Core> Intrinsics for C {}
pub trait Intrinsics: super::Core {
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
    fn _mm_setr_epi16() {
        hard_soft_same_128! {
            { let e7 = - 24391i16; let e6 = 19541i16; let e5 = - 16509i16; let e4 =
            7733i16; let e3 = - 15140i16; let e2 = 30719i16; let e1 = 16513i16; let e0 =
            22878i16; _mm_setr_epi16(e7, e6, e5, e4, e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_packs_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(23986i16, 27900i16, - 8343i16, - 10648i16, 4841i16,
            14610i16, - 17251i16, - 3971i16); let b = _mm_setr_epi16(22390i16, -
            23547i16, 15401i16, 15832i16, - 14212i16, - 1286i16, - 18062i16, 22296i16);
            _mm_packs_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packs_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(18077i16, 23617i16, - 9205i16, 21233i16, - 4332i16,
            - 31339i16, 23623i16, - 22080i16); let b = _mm_setr_epi16(- 1436i16, -
            30227i16, 8629i16, 10922i16, - 16731i16, - 1013i16, - 14310i16, 2892i16);
            _mm_packs_epi32(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 28568i16, 12614i16, 20103i16, 32412i16, -
            28704i16, - 27930i16, 4197i16, 1829i16); let b = _mm_setr_epi16(9149i16,
            18759i16, 30885i16, - 3879i16, 21600i16, 24454i16, 23524i16, 10765i16);
            _mm_packus_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(32539i16, 26890i16, - 3892i16, 4386i16, 18704i16,
            8253i16, - 29217i16, 32013i16); let b = _mm_setr_epi16(7448i16, 2172i16, -
            14764i16, - 1068i16, - 25463i16, 21215i16, - 31392i16, - 14015i16);
            _mm_packus_epi32(a, b) }
        }
    }
}


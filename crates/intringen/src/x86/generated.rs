#![allow(unused_parens)]
impl<C: super::Core> Intrinsics for C {}
pub trait Intrinsics: super::Core {
    fn _mm_set_epi64x(&mut self, dst: &mut Self::__m128i, e1: Self::u64, e0: Self::u64) {
        self.set_lane___m128i_u64(dst, 0u64, e0);
        self.set_lane___m128i_u64(dst, 1u64, e1);
    }
    fn _mm_setr_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        e3: Self::u32,
        e2: Self::u32,
        e1: Self::u32,
        e0: Self::u32,
    ) {
        self.set_lane___m128i_u32(dst, 0u64, e3);
        self.set_lane___m128i_u32(dst, 1u64, e2);
        self.set_lane___m128i_u32(dst, 2u64, e1);
        self.set_lane___m128i_u32(dst, 3u64, e0);
    }
    fn _mm_setr_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        e7: Self::u16,
        e6: Self::u16,
        e5: Self::u16,
        e4: Self::u16,
        e3: Self::u16,
        e2: Self::u16,
        e1: Self::u16,
        e0: Self::u16,
    ) {
        self.set_lane___m128i_u16(dst, 0u64, e7);
        self.set_lane___m128i_u16(dst, 1u64, e6);
        self.set_lane___m128i_u16(dst, 2u64, e5);
        self.set_lane___m128i_u16(dst, 3u64, e4);
        self.set_lane___m128i_u16(dst, 4u64, e3);
        self.set_lane___m128i_u16(dst, 5u64, e2);
        self.set_lane___m128i_u16(dst, 6u64, e1);
        self.set_lane___m128i_u16(dst, 7u64, e0);
    }
    fn _mm_setr_epi8(
        &mut self,
        dst: &mut Self::__m128i,
        e15: Self::u8,
        e14: Self::u8,
        e13: Self::u8,
        e12: Self::u8,
        e11: Self::u8,
        e10: Self::u8,
        e9: Self::u8,
        e8: Self::u8,
        e7: Self::u8,
        e6: Self::u8,
        e5: Self::u8,
        e4: Self::u8,
        e3: Self::u8,
        e2: Self::u8,
        e1: Self::u8,
        e0: Self::u8,
    ) {
        self.set_lane___m128i_u8(dst, 0u64, e15);
        self.set_lane___m128i_u8(dst, 1u64, e14);
        self.set_lane___m128i_u8(dst, 2u64, e13);
        self.set_lane___m128i_u8(dst, 3u64, e12);
        self.set_lane___m128i_u8(dst, 4u64, e11);
        self.set_lane___m128i_u8(dst, 5u64, e10);
        self.set_lane___m128i_u8(dst, 6u64, e9);
        self.set_lane___m128i_u8(dst, 7u64, e8);
        self.set_lane___m128i_u8(dst, 8u64, e7);
        self.set_lane___m128i_u8(dst, 9u64, e6);
        self.set_lane___m128i_u8(dst, 10u64, e5);
        self.set_lane___m128i_u8(dst, 11u64, e4);
        self.set_lane___m128i_u8(dst, 12u64, e3);
        self.set_lane___m128i_u8(dst, 13u64, e2);
        self.set_lane___m128i_u8(dst, 14u64, e1);
        self.set_lane___m128i_u8(dst, 15u64, e0);
    }
    fn _mm_packs_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_u16(a, 0u64);
        let __tmp1 = self.saturate8(__tmp0);
        self.set_lane___m128i_u8(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_u16(a, 1u64);
        let __tmp3 = self.saturate8(__tmp2);
        self.set_lane___m128i_u8(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_u16(a, 2u64);
        let __tmp5 = self.saturate8(__tmp4);
        self.set_lane___m128i_u8(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_u16(a, 3u64);
        let __tmp7 = self.saturate8(__tmp6);
        self.set_lane___m128i_u8(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_u16(a, 4u64);
        let __tmp9 = self.saturate8(__tmp8);
        self.set_lane___m128i_u8(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_u16(a, 5u64);
        let __tmp11 = self.saturate8(__tmp10);
        self.set_lane___m128i_u8(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_u16(a, 6u64);
        let __tmp13 = self.saturate8(__tmp12);
        self.set_lane___m128i_u8(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_u16(a, 7u64);
        let __tmp15 = self.saturate8(__tmp14);
        self.set_lane___m128i_u8(dst, 7u64, __tmp15);
        let __tmp16 = self.get_lane___m128i_u16(b, 0u64);
        let __tmp17 = self.saturate8(__tmp16);
        self.set_lane___m128i_u8(dst, 8u64, __tmp17);
        let __tmp18 = self.get_lane___m128i_u16(b, 1u64);
        let __tmp19 = self.saturate8(__tmp18);
        self.set_lane___m128i_u8(dst, 9u64, __tmp19);
        let __tmp20 = self.get_lane___m128i_u16(b, 2u64);
        let __tmp21 = self.saturate8(__tmp20);
        self.set_lane___m128i_u8(dst, 10u64, __tmp21);
        let __tmp22 = self.get_lane___m128i_u16(b, 3u64);
        let __tmp23 = self.saturate8(__tmp22);
        self.set_lane___m128i_u8(dst, 11u64, __tmp23);
        let __tmp24 = self.get_lane___m128i_u16(b, 4u64);
        let __tmp25 = self.saturate8(__tmp24);
        self.set_lane___m128i_u8(dst, 12u64, __tmp25);
        let __tmp26 = self.get_lane___m128i_u16(b, 5u64);
        let __tmp27 = self.saturate8(__tmp26);
        self.set_lane___m128i_u8(dst, 13u64, __tmp27);
        let __tmp28 = self.get_lane___m128i_u16(b, 6u64);
        let __tmp29 = self.saturate8(__tmp28);
        self.set_lane___m128i_u8(dst, 14u64, __tmp29);
        let __tmp30 = self.get_lane___m128i_u16(b, 7u64);
        let __tmp31 = self.saturate8(__tmp30);
        self.set_lane___m128i_u8(dst, 15u64, __tmp31);
    }
    fn _mm_packs_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_u32(a, 0u64);
        let __tmp1 = self.saturate16(__tmp0);
        self.set_lane___m128i_u16(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_u32(a, 1u64);
        let __tmp3 = self.saturate16(__tmp2);
        self.set_lane___m128i_u16(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_u32(a, 2u64);
        let __tmp5 = self.saturate16(__tmp4);
        self.set_lane___m128i_u16(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_u32(a, 3u64);
        let __tmp7 = self.saturate16(__tmp6);
        self.set_lane___m128i_u16(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_u32(b, 0u64);
        let __tmp9 = self.saturate16(__tmp8);
        self.set_lane___m128i_u16(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_u32(b, 1u64);
        let __tmp11 = self.saturate16(__tmp10);
        self.set_lane___m128i_u16(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_u32(b, 2u64);
        let __tmp13 = self.saturate16(__tmp12);
        self.set_lane___m128i_u16(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_u32(b, 3u64);
        let __tmp15 = self.saturate16(__tmp14);
        self.set_lane___m128i_u16(dst, 7u64, __tmp15);
    }
    fn _mm_packus_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_u16(a, 0u64);
        let __tmp1 = self.saturate_u8(__tmp0);
        self.set_lane___m128i_u8(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_u16(a, 1u64);
        let __tmp3 = self.saturate_u8(__tmp2);
        self.set_lane___m128i_u8(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_u16(a, 2u64);
        let __tmp5 = self.saturate_u8(__tmp4);
        self.set_lane___m128i_u8(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_u16(a, 3u64);
        let __tmp7 = self.saturate_u8(__tmp6);
        self.set_lane___m128i_u8(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_u16(a, 4u64);
        let __tmp9 = self.saturate_u8(__tmp8);
        self.set_lane___m128i_u8(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_u16(a, 5u64);
        let __tmp11 = self.saturate_u8(__tmp10);
        self.set_lane___m128i_u8(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_u16(a, 6u64);
        let __tmp13 = self.saturate_u8(__tmp12);
        self.set_lane___m128i_u8(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_u16(a, 7u64);
        let __tmp15 = self.saturate_u8(__tmp14);
        self.set_lane___m128i_u8(dst, 7u64, __tmp15);
        let __tmp16 = self.get_lane___m128i_u16(b, 0u64);
        let __tmp17 = self.saturate_u8(__tmp16);
        self.set_lane___m128i_u8(dst, 8u64, __tmp17);
        let __tmp18 = self.get_lane___m128i_u16(b, 1u64);
        let __tmp19 = self.saturate_u8(__tmp18);
        self.set_lane___m128i_u8(dst, 9u64, __tmp19);
        let __tmp20 = self.get_lane___m128i_u16(b, 2u64);
        let __tmp21 = self.saturate_u8(__tmp20);
        self.set_lane___m128i_u8(dst, 10u64, __tmp21);
        let __tmp22 = self.get_lane___m128i_u16(b, 3u64);
        let __tmp23 = self.saturate_u8(__tmp22);
        self.set_lane___m128i_u8(dst, 11u64, __tmp23);
        let __tmp24 = self.get_lane___m128i_u16(b, 4u64);
        let __tmp25 = self.saturate_u8(__tmp24);
        self.set_lane___m128i_u8(dst, 12u64, __tmp25);
        let __tmp26 = self.get_lane___m128i_u16(b, 5u64);
        let __tmp27 = self.saturate_u8(__tmp26);
        self.set_lane___m128i_u8(dst, 13u64, __tmp27);
        let __tmp28 = self.get_lane___m128i_u16(b, 6u64);
        let __tmp29 = self.saturate_u8(__tmp28);
        self.set_lane___m128i_u8(dst, 14u64, __tmp29);
        let __tmp30 = self.get_lane___m128i_u16(b, 7u64);
        let __tmp31 = self.saturate_u8(__tmp30);
        self.set_lane___m128i_u8(dst, 15u64, __tmp31);
    }
    fn _mm_packus_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_u32(a, 0u64);
        let __tmp1 = self.saturate_u16(__tmp0);
        self.set_lane___m128i_u16(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_u32(a, 1u64);
        let __tmp3 = self.saturate_u16(__tmp2);
        self.set_lane___m128i_u16(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_u32(a, 2u64);
        let __tmp5 = self.saturate_u16(__tmp4);
        self.set_lane___m128i_u16(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_u32(a, 3u64);
        let __tmp7 = self.saturate_u16(__tmp6);
        self.set_lane___m128i_u16(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_u32(b, 0u64);
        let __tmp9 = self.saturate_u16(__tmp8);
        self.set_lane___m128i_u16(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_u32(b, 1u64);
        let __tmp11 = self.saturate_u16(__tmp10);
        self.set_lane___m128i_u16(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_u32(b, 2u64);
        let __tmp13 = self.saturate_u16(__tmp12);
        self.set_lane___m128i_u16(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_u32(b, 3u64);
        let __tmp15 = self.saturate_u16(__tmp14);
        self.set_lane___m128i_u16(dst, 7u64, __tmp15);
    }
    fn _mm_abs_epi8(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=15u64 {
            let i = (j * 8u64);
            let __tmp0 = self.get_lane___m128i_u8(a, (i / 8u64));
            let __tmp1 = self.abs_u8(__tmp0);
            self.set_lane___m128i_u8(dst, (i / 8u64), __tmp1);
        }
    }
    fn _mm_abs_epi16(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=7u64 {
            let i = (j * 16u64);
            let __tmp0 = self.get_lane___m128i_u16(a, (i / 16u64));
            let __tmp1 = self.abs_u16(__tmp0);
            self.set_lane___m128i_u16(dst, (i / 16u64), __tmp1);
        }
    }
    fn _mm_abs_epi32(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=3u64 {
            let i = (j * 32u64);
            let __tmp0 = self.get_lane___m128i_u32(a, (i / 32u64));
            let __tmp1 = self.abs_u32(__tmp0);
            self.set_lane___m128i_u32(dst, (i / 32u64), __tmp1);
        }
    }
}
pub mod soft_arch {
    pub use super::super::soft_arch_types::*;
    use super::Intrinsics;
    pub fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_set_epi64x(&mut output, e1 as _, e0 as _);
        output
    }
    pub fn _mm_setr_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore
            ._mm_setr_epi32(&mut output, e3 as _, e2 as _, e1 as _, e0 as _);
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
            ._mm_setr_epi16(
                &mut output,
                e7 as _,
                e6 as _,
                e5 as _,
                e4 as _,
                e3 as _,
                e2 as _,
                e1 as _,
                e0 as _,
            );
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
                e15 as _,
                e14 as _,
                e13 as _,
                e12 as _,
                e11 as _,
                e10 as _,
                e9 as _,
                e8 as _,
                e7 as _,
                e6 as _,
                e5 as _,
                e4 as _,
                e3 as _,
                e2 as _,
                e1 as _,
                e0 as _,
            );
        output
    }
    pub fn _mm_packs_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packs_epi16(&mut output, a as _, b as _);
        output
    }
    pub fn _mm_packs_epi32(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packs_epi32(&mut output, a as _, b as _);
        output
    }
    pub fn _mm_packus_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packus_epi16(&mut output, a as _, b as _);
        output
    }
    pub fn _mm_packus_epi32(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_packus_epi32(&mut output, a as _, b as _);
        output
    }
    pub fn _mm_abs_epi8(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi8(&mut output, a as _);
        output
    }
    pub fn _mm_abs_epi16(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi16(&mut output, a as _);
        output
    }
    pub fn _mm_abs_epi32(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi32(&mut output, a as _);
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
    #[test]
    fn _mm_abs_epi8() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(6572i16, - 54i16, 10431i16, - 4614i16, - 1911i16,
            17046i16, - 12772i16, - 28109i16); _mm_abs_epi8(a) }
        }
    }
    #[test]
    fn _mm_abs_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(7409i16, - 30136i16, - 28607i16, - 1975i16,
            23451i16, - 32657i16, - 28920i16, - 2519i16); _mm_abs_epi16(a) }
        }
    }
    #[test]
    fn _mm_abs_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 7284i16, 7023i16, - 31688i16, 4770i16, 28846i16, -
            13549i16, 13781i16, - 10474i16); _mm_abs_epi32(a) }
        }
    }
}


#![allow(unused_parens)]
impl<C: super::Core> Intrinsics for C {}
pub trait Intrinsics: super::Core {
    fn _mm_add_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        for j in 0u64..=7u64 {
            let i = (j * 16u64);
            let __tmp0 = self.get_lane___m128i_u16(a, (i / 16u64));
            let __tmp1 = self.get_lane___m128i_u16(b, (i / 16u64));
            let __tmp2 = self.add_u16(__tmp0, __tmp1);
            self.set_lane___m128i_u16(dst, (i / 16u64), __tmp2);
        }
    }
    fn _mm_add_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        for j in 0u64..=3u64 {
            let i = (j * 32u64);
            let __tmp0 = self.get_lane___m128i_u32(a, (i / 32u64));
            let __tmp1 = self.get_lane___m128i_u32(b, (i / 32u64));
            let __tmp2 = self.add_u32(__tmp0, __tmp1);
            self.set_lane___m128i_u32(dst, (i / 32u64), __tmp2);
        }
    }
    fn _mm_add_epi64(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        for j in 0u64..=1u64 {
            let i = (j * 64u64);
            let __tmp0 = self.get_lane___m128i_u64(a, (i / 64u64));
            let __tmp1 = self.get_lane___m128i_u64(b, (i / 64u64));
            let __tmp2 = self.add_u64(__tmp0, __tmp1);
            self.set_lane___m128i_u64(dst, (i / 64u64), __tmp2);
        }
    }
    fn _mm_set_epi64x(&mut self, dst: &mut Self::__m128i, e1: Self::i64, e0: Self::i64) {
        let __tmp0 = self.cast_sign_i64_u64(e0);
        self.set_lane___m128i_u64(dst, 0u64, __tmp0);
        let __tmp1 = self.cast_sign_i64_u64(e1);
        self.set_lane___m128i_u64(dst, 1u64, __tmp1);
    }
    fn _mm_setr_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        e3: Self::i32,
        e2: Self::i32,
        e1: Self::i32,
        e0: Self::i32,
    ) {
        let __tmp0 = self.cast_sign_i32_u32(e3);
        self.set_lane___m128i_u32(dst, 0u64, __tmp0);
        let __tmp1 = self.cast_sign_i32_u32(e2);
        self.set_lane___m128i_u32(dst, 1u64, __tmp1);
        let __tmp2 = self.cast_sign_i32_u32(e1);
        self.set_lane___m128i_u32(dst, 2u64, __tmp2);
        let __tmp3 = self.cast_sign_i32_u32(e0);
        self.set_lane___m128i_u32(dst, 3u64, __tmp3);
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
        let __tmp0 = self.cast_sign_i16_u16(e7);
        self.set_lane___m128i_u16(dst, 0u64, __tmp0);
        let __tmp1 = self.cast_sign_i16_u16(e6);
        self.set_lane___m128i_u16(dst, 1u64, __tmp1);
        let __tmp2 = self.cast_sign_i16_u16(e5);
        self.set_lane___m128i_u16(dst, 2u64, __tmp2);
        let __tmp3 = self.cast_sign_i16_u16(e4);
        self.set_lane___m128i_u16(dst, 3u64, __tmp3);
        let __tmp4 = self.cast_sign_i16_u16(e3);
        self.set_lane___m128i_u16(dst, 4u64, __tmp4);
        let __tmp5 = self.cast_sign_i16_u16(e2);
        self.set_lane___m128i_u16(dst, 5u64, __tmp5);
        let __tmp6 = self.cast_sign_i16_u16(e1);
        self.set_lane___m128i_u16(dst, 6u64, __tmp6);
        let __tmp7 = self.cast_sign_i16_u16(e0);
        self.set_lane___m128i_u16(dst, 7u64, __tmp7);
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
        let __tmp0 = self.cast_sign_i8_u8(e15);
        self.set_lane___m128i_u8(dst, 0u64, __tmp0);
        let __tmp1 = self.cast_sign_i8_u8(e14);
        self.set_lane___m128i_u8(dst, 1u64, __tmp1);
        let __tmp2 = self.cast_sign_i8_u8(e13);
        self.set_lane___m128i_u8(dst, 2u64, __tmp2);
        let __tmp3 = self.cast_sign_i8_u8(e12);
        self.set_lane___m128i_u8(dst, 3u64, __tmp3);
        let __tmp4 = self.cast_sign_i8_u8(e11);
        self.set_lane___m128i_u8(dst, 4u64, __tmp4);
        let __tmp5 = self.cast_sign_i8_u8(e10);
        self.set_lane___m128i_u8(dst, 5u64, __tmp5);
        let __tmp6 = self.cast_sign_i8_u8(e9);
        self.set_lane___m128i_u8(dst, 6u64, __tmp6);
        let __tmp7 = self.cast_sign_i8_u8(e8);
        self.set_lane___m128i_u8(dst, 7u64, __tmp7);
        let __tmp8 = self.cast_sign_i8_u8(e7);
        self.set_lane___m128i_u8(dst, 8u64, __tmp8);
        let __tmp9 = self.cast_sign_i8_u8(e6);
        self.set_lane___m128i_u8(dst, 9u64, __tmp9);
        let __tmp10 = self.cast_sign_i8_u8(e5);
        self.set_lane___m128i_u8(dst, 10u64, __tmp10);
        let __tmp11 = self.cast_sign_i8_u8(e4);
        self.set_lane___m128i_u8(dst, 11u64, __tmp11);
        let __tmp12 = self.cast_sign_i8_u8(e3);
        self.set_lane___m128i_u8(dst, 12u64, __tmp12);
        let __tmp13 = self.cast_sign_i8_u8(e2);
        self.set_lane___m128i_u8(dst, 13u64, __tmp13);
        let __tmp14 = self.cast_sign_i8_u8(e1);
        self.set_lane___m128i_u8(dst, 14u64, __tmp14);
        let __tmp15 = self.cast_sign_i8_u8(e0);
        self.set_lane___m128i_u8(dst, 15u64, __tmp15);
    }
    fn _mm_packs_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_i16(a, 0u64);
        let __tmp1 = self.saturate8(__tmp0);
        self.set_lane___m128i_i8(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_i16(a, 1u64);
        let __tmp3 = self.saturate8(__tmp2);
        self.set_lane___m128i_i8(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_i16(a, 2u64);
        let __tmp5 = self.saturate8(__tmp4);
        self.set_lane___m128i_i8(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_i16(a, 3u64);
        let __tmp7 = self.saturate8(__tmp6);
        self.set_lane___m128i_i8(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_i16(a, 4u64);
        let __tmp9 = self.saturate8(__tmp8);
        self.set_lane___m128i_i8(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_i16(a, 5u64);
        let __tmp11 = self.saturate8(__tmp10);
        self.set_lane___m128i_i8(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_i16(a, 6u64);
        let __tmp13 = self.saturate8(__tmp12);
        self.set_lane___m128i_i8(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_i16(a, 7u64);
        let __tmp15 = self.saturate8(__tmp14);
        self.set_lane___m128i_i8(dst, 7u64, __tmp15);
        let __tmp16 = self.get_lane___m128i_i16(b, 0u64);
        let __tmp17 = self.saturate8(__tmp16);
        self.set_lane___m128i_i8(dst, 8u64, __tmp17);
        let __tmp18 = self.get_lane___m128i_i16(b, 1u64);
        let __tmp19 = self.saturate8(__tmp18);
        self.set_lane___m128i_i8(dst, 9u64, __tmp19);
        let __tmp20 = self.get_lane___m128i_i16(b, 2u64);
        let __tmp21 = self.saturate8(__tmp20);
        self.set_lane___m128i_i8(dst, 10u64, __tmp21);
        let __tmp22 = self.get_lane___m128i_i16(b, 3u64);
        let __tmp23 = self.saturate8(__tmp22);
        self.set_lane___m128i_i8(dst, 11u64, __tmp23);
        let __tmp24 = self.get_lane___m128i_i16(b, 4u64);
        let __tmp25 = self.saturate8(__tmp24);
        self.set_lane___m128i_i8(dst, 12u64, __tmp25);
        let __tmp26 = self.get_lane___m128i_i16(b, 5u64);
        let __tmp27 = self.saturate8(__tmp26);
        self.set_lane___m128i_i8(dst, 13u64, __tmp27);
        let __tmp28 = self.get_lane___m128i_i16(b, 6u64);
        let __tmp29 = self.saturate8(__tmp28);
        self.set_lane___m128i_i8(dst, 14u64, __tmp29);
        let __tmp30 = self.get_lane___m128i_i16(b, 7u64);
        let __tmp31 = self.saturate8(__tmp30);
        self.set_lane___m128i_i8(dst, 15u64, __tmp31);
    }
    fn _mm_packs_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_i32(a, 0u64);
        let __tmp1 = self.saturate16(__tmp0);
        self.set_lane___m128i_i16(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_i32(a, 1u64);
        let __tmp3 = self.saturate16(__tmp2);
        self.set_lane___m128i_i16(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_i32(a, 2u64);
        let __tmp5 = self.saturate16(__tmp4);
        self.set_lane___m128i_i16(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_i32(a, 3u64);
        let __tmp7 = self.saturate16(__tmp6);
        self.set_lane___m128i_i16(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_i32(b, 0u64);
        let __tmp9 = self.saturate16(__tmp8);
        self.set_lane___m128i_i16(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_i32(b, 1u64);
        let __tmp11 = self.saturate16(__tmp10);
        self.set_lane___m128i_i16(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_i32(b, 2u64);
        let __tmp13 = self.saturate16(__tmp12);
        self.set_lane___m128i_i16(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_i32(b, 3u64);
        let __tmp15 = self.saturate16(__tmp14);
        self.set_lane___m128i_i16(dst, 7u64, __tmp15);
    }
    fn _mm_packus_epi16(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_i16(a, 0u64);
        let __tmp1 = self.saturate_u8(__tmp0);
        self.set_lane___m128i_u8(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_i16(a, 1u64);
        let __tmp3 = self.saturate_u8(__tmp2);
        self.set_lane___m128i_u8(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_i16(a, 2u64);
        let __tmp5 = self.saturate_u8(__tmp4);
        self.set_lane___m128i_u8(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_i16(a, 3u64);
        let __tmp7 = self.saturate_u8(__tmp6);
        self.set_lane___m128i_u8(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_i16(a, 4u64);
        let __tmp9 = self.saturate_u8(__tmp8);
        self.set_lane___m128i_u8(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_i16(a, 5u64);
        let __tmp11 = self.saturate_u8(__tmp10);
        self.set_lane___m128i_u8(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_i16(a, 6u64);
        let __tmp13 = self.saturate_u8(__tmp12);
        self.set_lane___m128i_u8(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_i16(a, 7u64);
        let __tmp15 = self.saturate_u8(__tmp14);
        self.set_lane___m128i_u8(dst, 7u64, __tmp15);
        let __tmp16 = self.get_lane___m128i_i16(b, 0u64);
        let __tmp17 = self.saturate_u8(__tmp16);
        self.set_lane___m128i_u8(dst, 8u64, __tmp17);
        let __tmp18 = self.get_lane___m128i_i16(b, 1u64);
        let __tmp19 = self.saturate_u8(__tmp18);
        self.set_lane___m128i_u8(dst, 9u64, __tmp19);
        let __tmp20 = self.get_lane___m128i_i16(b, 2u64);
        let __tmp21 = self.saturate_u8(__tmp20);
        self.set_lane___m128i_u8(dst, 10u64, __tmp21);
        let __tmp22 = self.get_lane___m128i_i16(b, 3u64);
        let __tmp23 = self.saturate_u8(__tmp22);
        self.set_lane___m128i_u8(dst, 11u64, __tmp23);
        let __tmp24 = self.get_lane___m128i_i16(b, 4u64);
        let __tmp25 = self.saturate_u8(__tmp24);
        self.set_lane___m128i_u8(dst, 12u64, __tmp25);
        let __tmp26 = self.get_lane___m128i_i16(b, 5u64);
        let __tmp27 = self.saturate_u8(__tmp26);
        self.set_lane___m128i_u8(dst, 13u64, __tmp27);
        let __tmp28 = self.get_lane___m128i_i16(b, 6u64);
        let __tmp29 = self.saturate_u8(__tmp28);
        self.set_lane___m128i_u8(dst, 14u64, __tmp29);
        let __tmp30 = self.get_lane___m128i_i16(b, 7u64);
        let __tmp31 = self.saturate_u8(__tmp30);
        self.set_lane___m128i_u8(dst, 15u64, __tmp31);
    }
    fn _mm_packus_epi32(
        &mut self,
        dst: &mut Self::__m128i,
        a: Self::__m128i,
        b: Self::__m128i,
    ) {
        let __tmp0 = self.get_lane___m128i_i32(a, 0u64);
        let __tmp1 = self.saturate_u16(__tmp0);
        self.set_lane___m128i_u16(dst, 0u64, __tmp1);
        let __tmp2 = self.get_lane___m128i_i32(a, 1u64);
        let __tmp3 = self.saturate_u16(__tmp2);
        self.set_lane___m128i_u16(dst, 1u64, __tmp3);
        let __tmp4 = self.get_lane___m128i_i32(a, 2u64);
        let __tmp5 = self.saturate_u16(__tmp4);
        self.set_lane___m128i_u16(dst, 2u64, __tmp5);
        let __tmp6 = self.get_lane___m128i_i32(a, 3u64);
        let __tmp7 = self.saturate_u16(__tmp6);
        self.set_lane___m128i_u16(dst, 3u64, __tmp7);
        let __tmp8 = self.get_lane___m128i_i32(b, 0u64);
        let __tmp9 = self.saturate_u16(__tmp8);
        self.set_lane___m128i_u16(dst, 4u64, __tmp9);
        let __tmp10 = self.get_lane___m128i_i32(b, 1u64);
        let __tmp11 = self.saturate_u16(__tmp10);
        self.set_lane___m128i_u16(dst, 5u64, __tmp11);
        let __tmp12 = self.get_lane___m128i_i32(b, 2u64);
        let __tmp13 = self.saturate_u16(__tmp12);
        self.set_lane___m128i_u16(dst, 6u64, __tmp13);
        let __tmp14 = self.get_lane___m128i_i32(b, 3u64);
        let __tmp15 = self.saturate_u16(__tmp14);
        self.set_lane___m128i_u16(dst, 7u64, __tmp15);
    }
    fn _mm_abs_epi8(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=15u64 {
            let i = (j * 8u64);
            let __tmp0 = self.get_lane___m128i_i8(a, (i / 8u64));
            let __tmp1 = self.abs_i8(__tmp0);
            self.set_lane___m128i_u8(dst, (i / 8u64), __tmp1);
        }
    }
    fn _mm_abs_epi16(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=7u64 {
            let i = (j * 16u64);
            let __tmp0 = self.get_lane___m128i_i16(a, (i / 16u64));
            let __tmp1 = self.abs_i16(__tmp0);
            self.set_lane___m128i_u16(dst, (i / 16u64), __tmp1);
        }
    }
    fn _mm_abs_epi32(&mut self, dst: &mut Self::__m128i, a: Self::__m128i) {
        for j in 0u64..=3u64 {
            let i = (j * 32u64);
            let __tmp0 = self.get_lane___m128i_i32(a, (i / 32u64));
            let __tmp1 = self.abs_i32(__tmp0);
            self.set_lane___m128i_u32(dst, (i / 32u64), __tmp1);
        }
    }
}
pub mod soft_arch {
    pub use super::super::soft_arch_types::*;
    use super::Intrinsics;
    pub fn _mm_add_epi16(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_add_epi16(&mut output, a, b);
        output
    }
    pub fn _mm_add_epi32(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_add_epi32(&mut output, a, b);
        output
    }
    pub fn _mm_add_epi64(a: __m128i, b: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_add_epi64(&mut output, a, b);
        output
    }
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
    pub fn _mm_abs_epi8(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi8(&mut output, a);
        output
    }
    pub fn _mm_abs_epi16(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi16(&mut output, a);
        output
    }
    pub fn _mm_abs_epi32(a: __m128i) -> __m128i {
        let mut output = unsafe { std::mem::zeroed() };
        super::super::ValueCore._mm_abs_epi32(&mut output, a);
        output
    }
}
#[cfg(all(test, target_arch = "x86_64"))]
pub mod tests {
    use super::super::compare_test_helper::hard_soft_same_128;
    #[test]
    fn _mm_add_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 24391i16, 19541i16, - 16509i16, 7733i16, -
            15140i16, 30719i16, 16513i16, 22878i16); let b = _mm_setr_epi16(23986i16,
            27900i16, - 8343i16, - 10648i16, 4841i16, 14610i16, - 17251i16, - 3971i16);
            _mm_add_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_add_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(22390i16, - 23547i16, 15401i16, 15832i16, -
            14212i16, - 1286i16, - 18062i16, 22296i16); let b = _mm_setr_epi16(18077i16,
            23617i16, - 9205i16, 21233i16, - 4332i16, - 31339i16, 23623i16, - 22080i16);
            _mm_add_epi32(a, b) }
        }
    }
    #[test]
    fn _mm_add_epi64() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 1436i16, - 30227i16, 8629i16, 10922i16, -
            16731i16, - 1013i16, - 14310i16, 2892i16); let b = _mm_setr_epi16(- 28568i16,
            12614i16, 20103i16, 32412i16, - 28704i16, - 27930i16, 4197i16, 1829i16);
            _mm_add_epi64(a, b) }
        }
    }
    #[test]
    fn _mm_set_epi64x() {
        hard_soft_same_128! {
            { let e1 = - 1407335585757566417i64; let e0 = 6810649108177377822i64;
            _mm_set_epi64x(e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi32() {
        hard_soft_same_128! {
            { let e3 = 1012103333i32; let e2 = - 1086525223i32; let e1 = - 1399630752i32;
            let e0 = - 395616378i32; _mm_setr_epi32(e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi16() {
        hard_soft_same_128! {
            { let e7 = 23524i16; let e6 = 10765i16; let e5 = 32539i16; let e4 = 26890i16;
            let e3 = - 3892i16; let e2 = 4386i16; let e1 = 18704i16; let e0 = 8253i16;
            _mm_setr_epi16(e7, e6, e5, e4, e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_setr_epi8() {
        hard_soft_same_128! {
            { let e15 = - 33i8; let e14 = 13i8; let e13 = 24i8; let e12 = 124i8; let e11
            = 84i8; let e10 = - 44i8; let e9 = - 119i8; let e8 = - 33i8; let e7 = 96i8;
            let e6 = 65i8; let e5 = - 53i8; let e4 = - 48i8; let e3 = - 70i8; let e2 = -
            83i8; let e1 = 115i8; let e0 = 45i8; _mm_setr_epi8(e15, e14, e13, e12, e11,
            e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0) }
        }
    }
    #[test]
    fn _mm_packs_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 9518i16, - 29742i16, 10115i16, 1617i16, 13256i16,
            - 2379i16, 19254i16, 7533i16); let b = _mm_setr_epi16(- 17891i16, 30761i16,
            2539i16, 4135i16, 26713i16, 16348i16, - 21336i16, 3595i16);
            _mm_packs_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packs_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(6572i16, - 54i16, 10431i16, - 4614i16, - 1911i16,
            17046i16, - 12772i16, - 28109i16); let b = _mm_setr_epi16(7409i16, -
            30136i16, - 28607i16, - 1975i16, 23451i16, - 32657i16, - 28920i16, -
            2519i16); _mm_packs_epi32(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 7284i16, 7023i16, - 31688i16, 4770i16, 28846i16, -
            13549i16, 13781i16, - 10474i16); let b = _mm_setr_epi16(12050i16, - 782i16,
            8840i16, 8344i16, 9169i16, 303i16, - 6879i16, - 28778i16);
            _mm_packus_epi16(a, b) }
        }
    }
    #[test]
    fn _mm_packus_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 11301i16, 10802i16, 18689i16, 12867i16, 18892i16,
            20484i16, - 4754i16, - 28358i16); let b = _mm_setr_epi16(27422i16, -
            14791i16, - 32685i16, - 4504i16, - 19709i16, 1090i16, 1898i16, 11224i16);
            _mm_packus_epi32(a, b) }
        }
    }
    #[test]
    fn _mm_abs_epi8() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(27569i16, 26879i16, 11743i16, 1055i16, 5327i16, -
            1490i16, - 6436i16, 1056i16); _mm_abs_epi8(a) }
        }
    }
    #[test]
    fn _mm_abs_epi16() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 16744i16, 28829i16, 23772i16, - 31202i16, 9764i16,
            16146i16, 29119i16, 1909i16); _mm_abs_epi16(a) }
        }
    }
    #[test]
    fn _mm_abs_epi32() {
        hard_soft_same_128! {
            { let a = _mm_setr_epi16(- 4803i16, - 23533i16, - 22862i16, - 25389i16, -
            16117i16, - 21476i16, 30010i16, - 15743i16); _mm_abs_epi32(a) }
        }
    }
}


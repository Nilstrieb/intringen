impl<C: super::Core> Intrinsics for C {}
trait Intrinsics: super::Core {
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

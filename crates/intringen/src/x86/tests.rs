#[cfg(target_arch = "x86_64")]
mod x86_compare {
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

    #[test]
    fn _mm_setr_epi16() {
        hard_soft_same_128! {
            _mm_setr_epi16(0, -1, 100, 3535, 35, 2, i16::MIN, i16::MAX)
        };
    }

    #[test]
    fn _mm_packus_epi16() {
        hard_soft_same_128! {
            let a = _mm_setr_epi16(0x100, -1, 0, 0, 0, 0, 0, 0);
            let b = _mm_setr_epi16(0, 0, 0, 0, 0, 0, -1, 0x100);
            _mm_packus_epi16(a, b)
        }
    }
}

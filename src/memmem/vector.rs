use core::mem::size_of;

pub(crate) trait Vector: Copy + core::fmt::Debug {
    unsafe fn splat(byte: u8) -> Self;
    unsafe fn load_unaligned(data: *const u8) -> Self;
    unsafe fn movemask(self) -> u32;
    unsafe fn cmpeq(self, vector2: Self) -> Self;
    unsafe fn and(self, vector2: Self) -> Self;
}

#[cfg(all(feature = "std", target_arch = "x86_64"))]
mod x86avx {
    use super::Vector;
    use core::arch::x86_64::*;

    impl Vector for __m256i {
        #[inline(always)]
        unsafe fn splat(byte: u8) -> __m256i {
            _mm256_set1_epi8(byte as i8)
        }

        #[inline(always)]
        unsafe fn load_unaligned(data: *const u8) -> __m256i {
            _mm256_loadu_si256(data as *const __m256i)
        }

        #[inline(always)]
        unsafe fn movemask(self) -> u32 {
            _mm256_movemask_epi8(self) as u32
        }

        #[inline(always)]
        unsafe fn cmpeq(self, vector2: Self) -> __m256i {
            _mm256_cmpeq_epi8(self, vector2)
        }

        #[inline(always)]
        unsafe fn and(self, vector2: Self) -> __m256i {
            _mm256_and_si256(self, vector2)
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod x86sse {
    use super::Vector;
    use core::arch::x86_64::*;

    impl Vector for __m128i {
        #[inline(always)]
        unsafe fn splat(byte: u8) -> __m128i {
            _mm_set1_epi8(byte as i8)
        }

        #[inline(always)]
        unsafe fn load_unaligned(data: *const u8) -> __m128i {
            _mm_loadu_si128(data as *const __m128i)
        }

        #[inline(always)]
        unsafe fn movemask(self) -> u32 {
            _mm_movemask_epi8(self) as u32
        }

        #[inline(always)]
        unsafe fn cmpeq(self, vector2: Self) -> __m128i {
            _mm_cmpeq_epi8(self, vector2)
        }

        #[inline(always)]
        unsafe fn and(self, vector2: Self) -> __m128i {
            _mm_and_si128(self, vector2)
        }
    }
}

#include <stdio.h>
#include <intrin.h>
#include <immintrin.h>
#include <smmintrin.h>
#include <wmmintrin.h>
#ifdef __clang__
#include "avxintrin.h"
#include "avx2intrin.h"
#endif

typedef unsigned int u32;

u32 singleScalar(u32 count, u32 *input) {
    u32 sum = 0;
    for (u32 i = 0; i < count; ++i) {
        sum += input[i];
    }
    return sum;
}

// __attribute__ requires GCC libstdc++ https://wetmelon.github.io/clang-on-windows.html
u32 __attribute__((target("ssse3"))) singleSSE(u32 count, u32 *input) {
    __m128i sum = _mm_setzero_si128();

    for (u32 i = 0; i < count; i += 4) {
        sum = _mm_add_epi32(sum, _mm_load_si128((__m128i *) &input[i]));
    }

    sum = _mm_hadd_epi32(sum, sum);
    sum = _mm_hadd_epi32(sum, sum);

    return  _mm_cvtsi128_si32(sum);
}

// __attribute__ requires GCC libstdc++ https://wetmelon.github.io/clang-on-windows.html
u32 __attribute__((target("avx2"))) singleAVX(u32 count, u32 *input) {
    __m256i sum = _mm256_setzero_si256();

    for (u32 i = 0; i < count; i += 8) {
        sum = _mm256_add_epi32(sum, _mm256_loadu_si256((__m256i *) &input[i]));
    }

    sum = _mm256_hadd_epi32(sum, sum);
    sum = _mm256_hadd_epi32(sum, sum);
    __m256i sumS = _mm256_permute2x128_si256(sum, sum, 1 | (1 << 4));
    sum = _mm256_hadd_epi32(sum, sumS);

    return  _mm256_cvtsi256_si32(sum);
}

int main(int argC, char **argV) {
    printf("%d\n", singleSSE(4, (u32 []) {1, 2, 3, 4}));
    printf("%d", singleAVX(8, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4}));
    return 0;
}
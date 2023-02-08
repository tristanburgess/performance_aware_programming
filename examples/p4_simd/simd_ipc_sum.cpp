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

u32 quadScalarPtr(u32 count, u32 *input) {
    u32 sumA = 0;
    u32 sumB = 0;
    u32 sumC = 0;
    u32 sumD = 0;

    count /= 4;

    while (count--) {
        sumA += input[0];
        sumB += input[1];
        sumC += input[2];
        sumD += input[3];

        input += 4;
    }
    
    return sumA + sumB + sumC + sumD;
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

// __attribute__ requires GCC libstdc++ https://wetmelon.github.io/clang-on-windows.html
u32 __attribute__((target("avx2"))) dualAVX(u32 count, u32 *input) {
    __m256i sumA = _mm256_setzero_si256();
    __m256i sumB = _mm256_setzero_si256();

    for (u32 i = 0; i < count; i += 16) {
        sumA = _mm256_add_epi32(sumA, _mm256_loadu_si256((__m256i *) &input[i]));
        sumB = _mm256_add_epi32(sumB, _mm256_loadu_si256((__m256i *) &input[i + 8]));
    }

    __m256i sum = _mm256_add_epi32(sumA, sumB);

    sum = _mm256_hadd_epi32(sum, sum);
    sum = _mm256_hadd_epi32(sum, sum);
    __m256i sumS = _mm256_permute2x128_si256(sum, sum, 1 | (1 << 4));
    sum = _mm256_hadd_epi32(sum, sumS);

    return  _mm256_cvtsi256_si32(sum);
}

// __attribute__ requires GCC libstdc++ https://wetmelon.github.io/clang-on-windows.html
u32 __attribute__((target("avx2"))) quadAVX(u32 count, u32 *input) {
    __m256i sumA = _mm256_setzero_si256();
    __m256i sumB = _mm256_setzero_si256();
    __m256i sumC = _mm256_setzero_si256();
    __m256i sumD = _mm256_setzero_si256();

    for (u32 i = 0; i < count; i += 32) {
        sumA = _mm256_add_epi32(sumA, _mm256_loadu_si256((__m256i *) &input[i]));
        sumB = _mm256_add_epi32(sumB, _mm256_loadu_si256((__m256i *) &input[i + 8]));
        sumC = _mm256_add_epi32(sumC, _mm256_loadu_si256((__m256i *) &input[i + 8]));
        sumD = _mm256_add_epi32(sumD, _mm256_loadu_si256((__m256i *) &input[i + 8]));
    }

    __m256i sumAB = _mm256_add_epi32(sumA, sumB);
    __m256i sumCD = _mm256_add_epi32(sumC, sumD);
    __m256i sum = _mm256_add_epi32(sumAB, sumCD);

    sum = _mm256_hadd_epi32(sum, sum);
    sum = _mm256_hadd_epi32(sum, sum);
    __m256i sumS = _mm256_permute2x128_si256(sum, sum, 1 | (1 << 4));
    sum = _mm256_hadd_epi32(sum, sumS);

    return  _mm256_cvtsi256_si32(sum);
}

// __attribute__ requires GCC libstdc++ https://wetmelon.github.io/clang-on-windows.html
u32 __attribute__((target("avx2"))) quadAVXPtr(u32 count, u32 *input) {
    __m256i sumA = _mm256_setzero_si256();
    __m256i sumB = _mm256_setzero_si256();
    __m256i sumC = _mm256_setzero_si256();
    __m256i sumD = _mm256_setzero_si256();

    count /= 32;

    while (count--) {
        sumA = _mm256_add_epi32(sumA, _mm256_loadu_si256((__m256i *) &input[0]));
        sumB = _mm256_add_epi32(sumB, _mm256_loadu_si256((__m256i *) &input[8]));
        sumC = _mm256_add_epi32(sumC, _mm256_loadu_si256((__m256i *) &input[16]));
        sumD = _mm256_add_epi32(sumD, _mm256_loadu_si256((__m256i *) &input[24]));
        
        input += 32;
    }

    __m256i sumAB = _mm256_add_epi32(sumA, sumB);
    __m256i sumCD = _mm256_add_epi32(sumC, sumD);
    __m256i sum = _mm256_add_epi32(sumAB, sumCD);

    sum = _mm256_hadd_epi32(sum, sum);
    sum = _mm256_hadd_epi32(sum, sum);
    __m256i sumS = _mm256_permute2x128_si256(sum, sum, 1 | (1 << 4));
    sum = _mm256_hadd_epi32(sum, sumS);

    return  _mm256_cvtsi256_si32(sum);
}

int main(int argC, char **argV) {
    printf("%d\n", quadScalarPtr(32, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}));
    printf("%d\n", singleAVX(32, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}));
    printf("%d\n", dualAVX(32, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}));
    printf("%d\n", quadAVX(32, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}));
    printf("%d", quadAVXPtr(32, (u32 []) {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}));
    return 0;
}
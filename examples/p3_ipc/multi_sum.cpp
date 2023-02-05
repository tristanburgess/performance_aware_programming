typedef unsigned int u32;

// 0.99 adds/clock
u32 unroll2Scalar(u32 count, u32 *input) {
    u32 sum = 0;

    for (u32 i = 0; i < count; i += 2) {
        sum += input[i];
        sum += input[i + 1];
    }
    
    return sum;
}

// 1.27 adds/clock
u32 dualScalar(u32 count, u32 *input) {
    u32 sumA = 0;
    u32 sumB = 0;

    for (u32 i = 0; i < count; i += 2) {
        sumA += input[i];
        sumB += input[i + 1];
    }
    
    return sumA + sumB;
}

// 1.70 adds/clock
u32 quadScalar(u32 count, u32 *input) {
    u32 sumA = 0;
    u32 sumB = 0;
    u32 sumC = 0;
    u32 sumD = 0;

    for (u32 i = 0; i < count; i += 2) {
        sumA += input[i];
        sumB += input[i + 1];
        sumC += input[i + 2];
        sumD += input[i + 3];
    }
    
    return sumA + sumB + sumC + sumD;
}

// 1.95 adds/clock
// We will cover this later, but for now know that this form of loop
// unblocks some mechanisms of microarchitecture that would otherwise prevent the full
// possible throughput from being realized.
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

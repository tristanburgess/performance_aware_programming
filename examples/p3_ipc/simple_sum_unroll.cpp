typedef unsigned int u32;

u32 singleScalar(u32 count, u32 *input) {
    u32 sum = 0;

    for (u32 i = 0; i < count; ++i) {
        sum += input[i];
    }
    
    return sum;
}

u32 unroll2Scalar(u32 count, u32 *input) {
    u32 sum = 0;

    for (u32 i = 0; i < count; i += 2) {
        sum += input[i];
        sum += input[i + 1];
    }
    
    return sum;
}

u32 unroll4Scalar(u32 count, u32 *input) {
    u32 sum = 0;

    for (u32 i = 0; i < count; i += 4) {
        sum += input[i];
        sum += input[i + 1];
        sum += input[i + 2];
        sum += input[i + 3];
    }
    
    return sum;
}
typedef unsigned int u32;
u32 singleScalar(u32 count, u32 *input) {
    u32 sum = 0;
    for (u32 i = 0; i < count; ++i) {
        sum += input[i];
    }
    return sum;
}
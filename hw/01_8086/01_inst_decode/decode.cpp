#include <assert.h>
#include <stdio.h>

typedef unsigned char u8;

#define BITS 16
#define BYTES_TO_READ 2

const char* reg_table[] = {"al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di"};
const char* inst_string = "mov";

void decode(u8 buf[]) {
    u8 lo = buf[0];
    u8 hi = buf[1];

    // ensure we're seeing something from the class of MOV REG/MEM instructions
    assert((lo & 0XFC) == 0x88);

    u8 mod = (hi & 0xC0) >> 6;
    // ensure we're operating in register-to-register mode
    assert(mod == 0x3);

    u8 d = (lo & 0x2) >> 1;
    u8 w = lo & 0x1;

    u8 reg = (hi & 0x38) >> 3;
    u8 rm = hi & 0x7;

    // per docs, if d bit is 1, reg register is dest, if d bit is 0, rm register is dest.
    const char* reg_names[] = {reg_table[2 * rm + w], reg_table[2 * reg + w]};
    printf("%s %s, %s", inst_string, reg_names[d], reg_names[!d]);
}

int main(int arg_c, char **arg_v) {
    u8 buf[BYTES_TO_READ] = {0};

    FILE *fp = stdin;
    if (arg_c > 1) {
        fp = fopen(arg_v[1], "rb");
    }
    if (!fp) {
        fprintf(stderr, "error: file open failed: '%s'.\n", arg_v[1]);
        return 1;
    }

    printf("bits %d\n\n", BITS);

    size_t bytes_read = 0;
    while ((bytes_read = fread(buf, sizeof(u8), BYTES_TO_READ, fp)) == BYTES_TO_READ) {
        decode(buf);
        printf("\n");
    }
    assert(feof(fp));

    if (fp != stdin) {
        fclose(fp);
    }
}
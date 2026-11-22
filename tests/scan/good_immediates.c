/* fixture: forbidden byte patterns hidden inside immediates only —
 * the instruction-boundary decoder must pass this (no false positive) */
__attribute__((visibility("default"))) long f(long x) {
#if defined(__aarch64__)
    /* x9 = 0x00cd050f — its LE bytes are 0f 05 cd 00, i.e. the x86
     * syscall/int opcodes sitting inside an immediate field */
    __asm__ volatile("movz x9, #0x050f\n\t"
                     "movk x9, #0x00cd, lsl #16\n\t"
                     "add %0, %0, x9"
                     : "+r"(x) :: "x9");
#elif defined(__x86_64__)
    __asm__ volatile("movabsq $0xdead050f, %%rax\n\t"
                     "movl $0x80cd, %%ecx\n\t"
                     "addq %%rcx, %0"
                     : "+r"(x) :: "rax", "rcx");
#else
#error "no immediate-pattern fixture for this arch"
#endif
    return x;
}

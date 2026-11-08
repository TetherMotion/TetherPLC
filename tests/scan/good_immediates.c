/* fixture: forbidden byte patterns hidden inside immediates only —
 * the instruction-boundary decoder must pass this (no false positive) */
__attribute__((visibility("default"))) long f(long x) {
    __asm__ volatile("movabsq $0xdead050f, %%rax\n\t"
                     "movl $0x80cd, %%ecx\n\t"
                     "addq %%rcx, %0"
                     : "+r"(x) :: "rax", "rcx");
    return x;
}

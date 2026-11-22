/* fixture: contains a raw syscall instruction — must be rejected */
__attribute__((visibility("default"))) void f(void) {
#if defined(__aarch64__)
    __asm__ volatile("svc #0");
#elif defined(__x86_64__)
    __asm__ volatile("syscall");
#else
#error "no syscall-gate fixture for this arch"
#endif
}

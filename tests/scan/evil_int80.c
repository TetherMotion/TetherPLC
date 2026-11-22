/* fixture: contains a legacy interrupt gate — must be rejected.
 * x86 uses int 0x80; aarch64's only userspace gate is svc. */
__attribute__((visibility("default"))) void f(void) {
#if defined(__aarch64__)
    __asm__ volatile("svc #0");
#elif defined(__x86_64__)
    __asm__ volatile("int $0x80");
#else
#error "no interrupt-gate fixture for this arch"
#endif
}

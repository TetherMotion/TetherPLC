/* fixture: contains int 0x80 — must be rejected */
__attribute__((visibility("default"))) void f(void) {
    __asm__ volatile("int $0x80");
}

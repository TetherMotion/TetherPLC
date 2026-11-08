/* fixture: contains a raw syscall instruction — must be rejected */
__attribute__((visibility("default"))) void f(void) {
    __asm__ volatile("syscall");
}

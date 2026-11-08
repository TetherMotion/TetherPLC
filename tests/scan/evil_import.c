/* fixture: imports system() — PLT call outside the whitelist */
extern int system(const char*);
__attribute__((visibility("default"))) int f(void) { return system("x"); }

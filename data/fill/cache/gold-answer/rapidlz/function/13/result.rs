pub fn RapidlzLogRegister(func: RapidlzLogFunc) {
    *g_rapidlzLogFunc.lock() = func;
}
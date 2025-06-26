pub fn RapidlzLogRegister(mut func: RapidlzLogFunc) {
    *g_rapidlzLogFunc.lock() = func.cast();
}

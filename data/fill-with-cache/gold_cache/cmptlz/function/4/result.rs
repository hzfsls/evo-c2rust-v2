pub fn CmptlzLogRegister(func: CmptlzLogFunc) {
    *g_cmptlzLogFunc.lock() = func;
}
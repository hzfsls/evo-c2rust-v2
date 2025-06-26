pub fn CmptlzLogRegister(mut func: CmptlzLogFunc) {
    *g_cmptlzLogFunc.lock() = func.cast();
}

pub fn rapidlz_log_register(func: RapidlzLogFunc) {
    unsafe {
        g_rapidlz_log_func = func;
    }
}

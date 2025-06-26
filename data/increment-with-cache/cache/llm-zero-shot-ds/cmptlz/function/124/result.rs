pub static mut g_cmptlz_log_func: Option<CmptlzLogFunc> = None;

pub fn cmptlz_log_register(func: CmptlzLogFunc) {
    unsafe {
        g_cmptlz_log_func = Some(func);
    }
}

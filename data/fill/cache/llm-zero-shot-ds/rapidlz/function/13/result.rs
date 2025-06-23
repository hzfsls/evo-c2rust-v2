static mut G_RAPIDLZ_LOG_FUNC: Option<RapidlzLogFunc> = None;

pub type RapidlzLogFunc = fn(/* appropriate parameters */); // Replace with actual function signature

pub fn rapidlz_log_register(func: RapidlzLogFunc) {
    unsafe {
        G_RAPIDLZ_LOG_FUNC = Some(func);
    }
}

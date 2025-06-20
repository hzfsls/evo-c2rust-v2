pub fn RapidlzLogWrite(
    mut error_code: usize,
    mut file_name: Ptr<u8>,
    mut line: u16,
    mut fmt: Ptr<u8>,
    mut alist: VaList,
) {
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut retVal: i32;
    let mut len: i32;
    let mut func: RapidlzLogFunc = *g_rapidlzLogFunc.lock();
    let mut filename: Ptr<u8>;
    if func == NULL!() {
        return;
    }
    filename = c_strdup!(file_name);
    if filename == NULL!() {
        return;
    }
    retVal = c_snprintf_s!(
        output,
        LOG_BUF_SIZE!(),
        LOG_BUF_SIZE!() - 1,
        cstr!("[Rapidlz-Log] File={}, Line={}, Error={}\n"),
        c_basename!(filename),
        line,
        error_code
    );
    if retVal < 0 {
        c_free!(filename);
        return;
    }
    len = retVal;
    c_free!(filename);
    retVal = c_vsnprintf_s!(
        output.cast::<Ptr<u8>>() + len,
        LOG_BUF_SIZE!() - len,
        LOG_BUF_SIZE!() - len - 1,
        fmt,
        alist
    );
    if retVal < 0 {
        return;
    }
    func(output.cast(), c_strlen!(output) + 1);
}
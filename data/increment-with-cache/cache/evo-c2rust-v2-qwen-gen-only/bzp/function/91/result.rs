pub fn BzpMtfFinish(mut mtf: Ptr<BzpMtfInfo>) {
    if (mtf != NULL!()).as_bool() {
        if (mtf.mtfV != NULL!()).as_bool() {
            c_free!(mtf.mtfV.cast());
            mtf.mtfV = NULL!();
        }
        c_free!(mtf.cast());
        mtf = NULL!();
    }
}
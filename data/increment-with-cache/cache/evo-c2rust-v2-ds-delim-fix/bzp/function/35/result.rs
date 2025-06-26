pub fn BzpMtfFinish(mut mtf: Ptr<BzpMtfInfo>) {
    if (mtf != NULL!()).as_bool() {
        if (mtf.mtfV != NULL!()).as_bool() {
            c_free!(mtf.mtfV);
            mtf.mtfV = NULL!();
        }
        c_free!(mtf);
        mtf = NULL!();
    }
}

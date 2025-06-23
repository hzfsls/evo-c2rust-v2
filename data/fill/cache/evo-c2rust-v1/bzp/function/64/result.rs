pub fn BzpMtfFinish(mut mtf: Ptr<BzpMtfInfo>) {
    if mtf != NULL!() {
        if mtf.mtfV != NULL!() {
            c_free!(mtf.mtfV);
            mtf.mtfV = NULL!();
        }
        c_free!(mtf);
        mtf = NULL!();
    }
}

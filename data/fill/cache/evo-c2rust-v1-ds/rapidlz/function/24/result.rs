pub fn RapidlzCCtxFree(mut cCtx: Ptr<RapidlzCCtx>) {
    if cCtx != NULL!() {
        if cCtx.hashTable != NULL!() {
            c_free!(cCtx.hashTable);
            cCtx.hashTable = NULL!();
        }
        c_free!(cCtx);
    }
}

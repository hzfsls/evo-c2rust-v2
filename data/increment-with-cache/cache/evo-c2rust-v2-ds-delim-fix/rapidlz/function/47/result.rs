pub fn RapidlzCCtxFree(mut cCtx: Ptr<RapidlzCCtx>) {
    if (cCtx != NULL!()).as_bool() {
        if (cCtx.hashTable != NULL!()).as_bool() {
            c_free!(cCtx.hashTable);
            cCtx.hashTable = NULL!();
        }
        c_free!(cCtx);
    }
}

pub fn CmptLzDecConstruct(mut decCtx: Ptr<CmptLzDecCtx>) {
    decCtx.dict = NULL!();
    decCtx.probs = NULL!();
}

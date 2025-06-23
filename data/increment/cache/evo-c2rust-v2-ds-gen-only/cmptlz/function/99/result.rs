pub fn CmptLzDecCheckDictSizeUpdate(mut decCtx: Ptr<CmptLzDecCtx>) {
    if (decCtx.checkDicSize == 0).as_bool() && (decCtx.processedPos >= decCtx.prop.dicSize).as_bool() {
        decCtx.checkDicSize = decCtx.prop.dicSize.cast();
    }
}

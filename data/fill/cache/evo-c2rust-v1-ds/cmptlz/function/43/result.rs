pub fn CmptLzDecCheckDictSizeUpdate(mut decCtx: Ptr<CmptLzDecCtx>) {
    if decCtx.checkDicSize == 0 && decCtx.processedPos >= decCtx.prop.dicSize {
        decCtx.checkDicSize = decCtx.prop.dicSize.cast();
    }
}

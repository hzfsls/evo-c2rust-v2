#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnCmptLzMemType {
    CmptlzDictMem = 1,
    CmptlzProbMem,
    CmptlzEncCctx,
    CmptlzMfCctx,
    CmptlzMfHash,
    CmptlzMfSon,
    CmptlzRcCctx,
    CmptlzRcBuf,
    CmptlzMemTypeBut,
}

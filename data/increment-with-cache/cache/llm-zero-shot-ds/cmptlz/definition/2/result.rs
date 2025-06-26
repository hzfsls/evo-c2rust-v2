#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnCmptErrNo {
    CmptlzErrorData = 1,
    CmptlzErrorMem,
    CmptlzErrorUnsupported,
    CmptlzEncErrorFilesize,
    CmptlzEncCtxInitFail,
    CmptlzEncRcInitFail,
    CmptlzEncMfInitFail,
    CmptlzEncErrorWrite,
    CmptlzEncErrorHead,
    CmptlzEncErrorParam,
    CmptlzErrorButt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnCmptLzStatus {
    NotSpecified,
    FinishedWithMark,
    NotFinished,
    NeedsMoreInput,
    MaybeFinishedWithoutMark,
    But,
}

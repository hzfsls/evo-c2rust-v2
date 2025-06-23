#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmptlzState {
    LitLit,
    MatchLitLit,
    RepLitLit,
    ShortrepLitLit,
    MatchLit,
    RepLit,
    ShortrepLit,
    LitMatch,
    LitLongrep,
    LitShortrep,
    NotlitMatch,
    NotlitRep,
}

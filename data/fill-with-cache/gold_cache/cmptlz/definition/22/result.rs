pub type CmptlzState = i32;
macro_rules! LIT_LIT { () => { 0 } }
macro_rules! MATCH_LIT_LIT { () => { 1 } }
macro_rules! REP_LIT_LIT { () => { 2 } }
macro_rules! SHORTREP_LIT_LIT { () => { 3 } }
macro_rules! MATCH_LIT { () => { 4 } }
macro_rules! REP_LIT { () => { 5 } }
macro_rules! SHORTREP_LIT { () => { 6 } }
macro_rules! LIT_MATCH { () => { 7 } }
macro_rules! LIT_LONGREP { () => { 8 } }
macro_rules! LIT_SHORTREP { () => { 9 } }
macro_rules! NOTLIT_MATCH { () => { 10 } }
macro_rules! NOTLIT_REP { () => { 11 } }
pub(crate) use LIT_LIT;
pub(crate) use MATCH_LIT_LIT;
pub(crate) use REP_LIT_LIT;
pub(crate) use SHORTREP_LIT_LIT;
pub(crate) use MATCH_LIT;
pub(crate) use REP_LIT;
pub(crate) use SHORTREP_LIT;
pub(crate) use LIT_MATCH;
pub(crate) use LIT_LONGREP;
pub(crate) use LIT_SHORTREP;
pub(crate) use NOTLIT_MATCH;
pub(crate) use NOTLIT_REP;
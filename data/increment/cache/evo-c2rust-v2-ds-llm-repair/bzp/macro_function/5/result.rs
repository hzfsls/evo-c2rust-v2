macro_rules! BZP_BUFF_READ_EMPTY { ($bzpf:expr) => { $bzpf.input.pos >= $bzpf.input.nBuf } }
pub(crate) use BZP_BUFF_READ_EMPTY;

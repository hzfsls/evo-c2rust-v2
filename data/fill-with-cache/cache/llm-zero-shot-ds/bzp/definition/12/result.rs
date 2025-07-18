pub const BZP_MAX_GROUPS_NUM: usize = 6;
pub const BZP_MAX_ALPHA_SIZE: usize = 258;

#[derive(Debug)]
pub struct BzpHuffmanDecode {
    pub select: Option<Box<[i32]>>,
    pub len: [[i32; BZP_MAX_ALPHA_SIZE]; BZP_MAX_GROUPS_NUM],
    pub perm: [[i32; BZP_MAX_ALPHA_SIZE]; BZP_MAX_GROUPS_NUM],
    pub limit: [[i32; BZP_MAX_ALPHA_SIZE]; BZP_MAX_GROUPS_NUM],
    pub base: [[i32; BZP_MAX_ALPHA_SIZE]; BZP_MAX_GROUPS_NUM],
    pub minLens: [i32; BZP_MAX_GROUPS_NUM],
    pub nGroups: i32,
    pub nSelect: i32,
    pub alphaSize: i32,
    pub deCodeNum: i32,
    pub selectCnt: i32,
    pub nBlock: i32,
}

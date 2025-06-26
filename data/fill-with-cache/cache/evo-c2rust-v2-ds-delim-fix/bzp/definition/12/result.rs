#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpHuffmanDecode {
    pub select: Ptr<i32>,
    pub len: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub perm: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub limit: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub base: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub minLens: Array<i32, { BZP_MAX_GROUPS_NUM!() }>,
    pub nGroups: i32,
    pub nSelect: i32,
    pub alphaSize: i32,
    pub deCodeNum: i32,
    pub selectCnt: i32,
    pub nBlock: i32,
}

#[repr(C)]
#[derive(Default)]
pub struct BzpHuffmanGroups {
    pub block: Ptr<i32>,
    pub mtfFreq: Ptr<i32>,
    pub select: Ptr<i32>,
    pub selectMTF: Ptr<i32>,
    pub huffmanGroups: Array<BzpHuffmanInfo, { BZP_MAX_GROUPS_NUM!() }>,
    pub cost: Array<i32, { BZP_MAX_GROUPS_NUM!() }>,
    pub nGroups: i32,
    pub nBlock: i32,
    pub nSelect: i32,
    pub alphaSize: i32,
}

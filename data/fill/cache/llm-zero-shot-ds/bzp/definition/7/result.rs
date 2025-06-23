pub struct BzpQSortInfo {
    pub stack_l: [i32; BZP_MAX_STACK_SIZE],
    pub stack_r: [i32; BZP_MAX_STACK_SIZE],
    pub cnt: i32,
    pub tl: i32,
    pub tr: i32,
}

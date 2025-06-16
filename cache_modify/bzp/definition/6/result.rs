#[repr(C)]
#[derive(Default)]
pub struct BzpQSortInfo {
    pub stackL: Array<i32, { BZP_MAX_STACK_SIZE!() }>,
    pub stackR: Array<i32, { BZP_MAX_STACK_SIZE!() }>,
    pub cnt: i32,
    pub tl: i32,
    pub tr: i32,
}

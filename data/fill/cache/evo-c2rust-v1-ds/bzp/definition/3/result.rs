#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpFile {
    pub input: Ptr<BzpStream>,
    pub output: Ptr<BzpStream>,
    pub state: i32,
    pub lasChar: i32,
    pub num: i32,
    pub pad: i32,
}

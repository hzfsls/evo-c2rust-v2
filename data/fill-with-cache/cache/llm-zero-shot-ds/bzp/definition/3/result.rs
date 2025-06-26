pub struct BzpFile {
    pub input: *mut BzpStream,
    pub output: *mut BzpStream,
    pub state: i32,
    pub las_char: i32,
    pub num: i32,
    pub pad: i32,
}

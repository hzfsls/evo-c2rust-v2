pub fn CmptlzIsLE() -> i32 {
    let mut n: i32 = 1;
    return (*c_ref!(n).cast::<Ptr<u8>>())[0];
}
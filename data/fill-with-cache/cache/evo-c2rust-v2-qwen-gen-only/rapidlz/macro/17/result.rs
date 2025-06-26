macro_rules! RAPIDLZFILENAME {
    () => {
        {
            let __file__ = __FILE__!();
            let __slash__ = strrchr(__file__ as *const u8, b'/');
            if __slash__.as_bool() {
                __slash__ + 1
            } else {
                __file__
            }
        }
    }
}
pub(crate) use RAPIDLZFILENAME;
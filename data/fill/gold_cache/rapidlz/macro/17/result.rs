macro_rules! RAPIDLZFILENAME {
    () => {
        if strrchr(c__file__!(), b'/').as_bool() {
            strrchr(c__file__!(), b'/') + 1
        } else {
            c__file__!()
        }
    };
}
pub(crate) use RAPIDLZFILENAME;
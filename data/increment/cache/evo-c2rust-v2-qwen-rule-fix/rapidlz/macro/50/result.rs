macro_rules! RAPIDLZFILENAME {
    () => {
        {
            let file = __FILE__!();
            let last_slash = strrchr(file, b'/');
            if last_slash.as_bool() {
                last_slash + 1
            } else {
                file
            }
        }
    }
}
pub(crate) use RAPIDLZFILENAME;
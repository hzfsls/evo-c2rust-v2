macro_rules! RAPIDLZFILENAME {
    () => {
        {
            let file = file!();
            match file.rfind('/') {
                Some(pos) => &file[pos + 1..],
                None => file,
            }
        }
    };
}

pub(crate) use RAPIDLZFILENAME;

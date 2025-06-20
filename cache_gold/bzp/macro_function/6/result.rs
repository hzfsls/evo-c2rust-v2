macro_rules! BZP_UPDATE_CRC {
    ($crcVar:expr, $cha:expr) => {
        $crcVar = ($crcVar << 8) ^ g_bzpCRC32Table.lock()[(($crcVar >> 24) ^ ($cha as u32)) as u8];
    }
}
pub(crate) use BZP_UPDATE_CRC;
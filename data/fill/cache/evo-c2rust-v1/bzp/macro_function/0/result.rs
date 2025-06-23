macro_rules! BZP_INVALID_BLOCK_SIZE { ($blockSize:expr) => 
    { 
        $blockSize < BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!() || $blockSize > BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!() 
    } 
}
pub(crate) use BZP_INVALID_BLOCK_SIZE;

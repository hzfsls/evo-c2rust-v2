macro_rules! MD5_RECORD_MESSAGE_LEN {
    ($context:expr) => {
        {
            for __i in 0..($context.aulCount.len()) {
                $context.aucBuffer[$context.uiPos] = (($context.aulCount[__i] & 0xff) as u8);
                $context.uiPos += 1;
                $context.aucBuffer[$context.uiPos] = ((($context.aulCount[__i] >> 8) & 0xff) as u8);
                $context.uiPos += 1;
                $context.aucBuffer[$context.uiPos] = ((($context.aulCount[__i] >> 16) & 0xff) as u8);
                $context.uiPos += 1;
                $context.aucBuffer[$context.uiPos] = ((($context.aulCount[__i] >> 24) & 0xff) as u8);
                $context.uiPos += 1;
            }
        }
    };
}

pub(crate) use MD5_RECORD_MESSAGE_LEN;

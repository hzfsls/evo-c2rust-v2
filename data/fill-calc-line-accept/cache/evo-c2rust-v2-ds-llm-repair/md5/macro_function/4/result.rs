macro_rules! MD5_RECORD_MESSAGE_LEN { ($context:expr) =>
    {
        let mut __i: u32;
        c_for!(__i = 0; __i < (c_sizeofval!($context.aulCount) / c_sizeofval!($context.aulCount[0])).cast(); __i.plus_plus(); {
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = ($context.aulCount[__i] & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 8) & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 16) & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 24) & 0xff) as u8;
            $context.uiPos += 1;
        });
    }
}
pub(crate) use MD5_RECORD_MESSAGE_LEN;

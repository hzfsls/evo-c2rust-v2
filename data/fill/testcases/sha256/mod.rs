pub mod sha256_c;

use sha256_c::*;

use crate::translation_utils::*;


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! UT_TEXT_LESS_56 {
        () => {
            bstr!(b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc\0")
        };
    }
    macro_rules! UT_TEXT_EQUAL_56 {
        () => {
            bstr!(b"12345678901234567890123456789012345678901234567890123456\0")
        };
    }
    macro_rules! UT_TEXT_LESS_64 { () => { bstr!(b"\xe5\x95\x8a\xe6\xb3\xa2\xe6\xac\xa1\xe7\x9a\x84\xe9\xa2\x9d\xe4\xbd\x9b\xe6\xad\x8c\xe9\xa2\x9d\xe5\xb7\xb2\xe7\xba\xa7\xe5\x8f\xaf\xe4\xba\x86\xe8\x8e\xab\xe5\x91\xa2\xe5\x93\xa6\xe7\xa0\xb4\xe5\x99\xa8\xe9\x88\xa4\xe6\x96\xaf\xe7\x89\xb9\xe5\x94\x94\0")} }
    macro_rules! UT_TEXT_EQUAL_64 {
        () => {
            bstr!(b"./,!@#$%^&*(){}[\"``~..\"/.;':<>?';:,./:>?:*&^$##%&^&^$!@;.!@#$%^&\0")
        };
    }
    macro_rules! UT_TEXT_MORE_64 { () => { bstr!(b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%\0")} }
    macro_rules! UT_TEXT_EQUAL_128 { () => { bstr!(b"a123b\"lmnop\xe8\x9b\x8b7345&^$##6\xe9\xa5\xad4^&5*&^$##\xe7\xb3\x96%678o12`~..3pq&^$#rst#%\"ef&5*&^$#896\xe7\x89\x9b7345&^$##6\xe7\xbe\x8a4^&5*&^$##%678o\xe7\x8c\xaa12`~..3pq&^$#rst#%\0")} }

    const digest_equal_0: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55
    ];

    const digest_less_56: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0x59, 0x56, 0x15, 0xdb, 0xe4, 0xf0, 0xf4, 0x07, 0xae, 0x39, 0x7d, 0x08, 0xb4, 0xc2, 0xcb,
        0x87, 0x0c, 0xb9, 0xb0, 0xe1, 0x19, 0x37, 0x41, 0x6f, 0x95, 0x0c, 0x51, 0x60, 0xac, 0xf9,
        0xc0, 0x05
    ];

    const digest_equal_56: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0x0b, 0xe6, 0x6c, 0xe7, 0x2c, 0x24, 0x67, 0xe7, 0x93, 0x20, 0x29, 0x06, 0x00, 0x06, 0x72,
        0x30, 0x66, 0x61, 0x79, 0x16, 0x22, 0xe0, 0xca, 0x9a, 0xdf, 0x4a, 0x89, 0x55, 0xb2, 0xed,
        0x18, 0x9c
    ];

    const digest_less_64: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0xa7, 0xe6, 0x34, 0xc0, 0xd0, 0xb2, 0x73, 0x07, 0x3d, 0xb0, 0x8e, 0xf1, 0xde, 0x15, 0xae,
        0xe0, 0x45, 0xa1, 0xc3, 0xfb, 0x76, 0x1f, 0x2f, 0xea, 0x7d, 0x91, 0x38, 0x65, 0x8e, 0x34,
        0x2d, 0x7f
    ];

    const digest_equal_64: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0x07, 0x5a, 0x5e, 0x09, 0x81, 0x95, 0xe5, 0x77, 0xd4, 0x06, 0xfa, 0xbe, 0x02, 0x17, 0x4f,
        0x90, 0xab, 0xdc, 0x68, 0x56, 0x13, 0x5b, 0xed, 0x22, 0x20, 0x9f, 0x27, 0x45, 0xd1, 0x3f,
        0x13, 0x73
    ];

    const digest_more_64: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0x81, 0x87, 0x77, 0xda, 0x19, 0x7e, 0x51, 0x67, 0x6b, 0xb5, 0x45, 0xca, 0x9f, 0x43, 0xbe,
        0x67, 0xb2, 0x7d, 0xc5, 0xf3, 0x5a, 0x72, 0x46, 0x13, 0x01, 0xde, 0xf9, 0x4e, 0x27, 0x0e,
        0x29, 0x60
    ];

    const digest_equal_128: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
        0x48, 0x3d, 0x04, 0xab, 0x6c, 0x34, 0x6c, 0x0b, 0x0f, 0x77, 0x04, 0x8f, 0xc2, 0x4a, 0xfe,
        0x92, 0x61, 0x87, 0x13, 0x63, 0x2c, 0x10, 0xb0, 0x88, 0x77, 0x64, 0xe3, 0x53, 0x73, 0x4d,
        0xae, 0x4b
    ];

    fn Test_DigestCmp(aucDigest1: Ptr<u8>, aucDigest2: Ptr<u8>, uiDigestLen: u32) -> u32 {
        let mut i = 0;
        while i < uiDigestLen {
            if aucDigest1[i] != aucDigest2[i] {
                return 1;
            }
            i += 1;
        }
        0
    }

    #[test]
    fn UT_VosSha256Calc001() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_LESS_56!();
        let uiInlen: u32 = 0;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_equal_0.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc002() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_LESS_56!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_less_56.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc003() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_EQUAL_56!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_equal_56.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc004() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_LESS_64!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_less_64.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc005() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_EQUAL_64!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_equal_64.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc006() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_MORE_64!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_more_64.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn UT_VosSha256Calc007() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = UT_TEXT_EQUAL_128!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        VOS_Sha256Calc(aucInput, uiInlen, aucOutput.cast(), SHA256_DIGEST_SIZE!());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_equal_128.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    macro_rules! SDV_TEXT_LESS_56 {
        () => {
            bstr!(b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc\0")
        };
    }

    macro_rules! SDV_TEXT_MORE_64 { () => { bstr!(b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%\0")} }

    #[test]
    fn SDV_VosSha256Calc001() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = cstr!("abcdefg");
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        let mut stCtx: Array<VOS_SHA256_CTX, 1> = Default::default();
        let mut digest_less_64_1: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
            0x90, 0x0e, 0xac, 0x36, 0xee, 0x15, 0xdb, 0x25, 0xd5, 0x2b, 0x80, 0x66, 0xfe, 0x82,
            0x78, 0xff, 0x2e, 0xcf, 0xc5, 0x73, 0x53, 0x0f, 0x1d, 0x2c, 0x9b, 0x72, 0x4b, 0x30,
            0x77, 0x90, 0x0c, 0x3e
        ];
        vosSha256Begin(stCtx.cast());
        vosSha256Hash(aucInput, uiInlen, stCtx.cast());
        vosSha256Hash(aucInput, uiInlen, stCtx.cast());
        vosSha256End(aucOutput.cast(), SHA256_DIGEST_SIZE!(), stCtx.cast());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_less_64_1.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn SDV_VosSha256Calc002() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput1: Ptr<u8> = SDV_TEXT_LESS_56!();
        let mut aucInput2: Ptr<u8> = cstr!("abcdefghi");
        let uiInlen1: u32 = c_strlen!(aucInput1) as u32;
        let uiInlen2: u32 = c_strlen!(aucInput2) as u32;
        let mut stCtx: Array<VOS_SHA256_CTX, 1> = Default::default();
        let mut digest_equal_64_2: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
            0x3c, 0x6e, 0xdb, 0xa7, 0x94, 0x67, 0x0b, 0x44, 0x41, 0xd1, 0xf3, 0xa8, 0x22, 0x5f,
            0xef, 0x18, 0x35, 0xcb, 0x84, 0x26, 0xf0, 0xc1, 0x80, 0xa0, 0xb9, 0x56, 0x23, 0x25,
            0x36, 0x4d, 0xac, 0xcc
        ];
        vosSha256Begin(stCtx.cast());
        vosSha256Hash(aucInput1, uiInlen1, stCtx.cast());
        vosSha256Hash(aucInput2, uiInlen2, stCtx.cast());
        vosSha256End(aucOutput.cast(), SHA256_DIGEST_SIZE!(), stCtx.cast());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_equal_64_2.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn SDV_VosSha256Calc003() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput1: Ptr<u8> = SDV_TEXT_LESS_56!();
        let mut aucInput2: Ptr<u8> = SDV_TEXT_LESS_56!();
        let uiInlen1: u32 = c_strlen!(aucInput1) as u32;
        let uiInlen2: u32 = c_strlen!(aucInput2) as u32;
        let mut stCtx: Array<VOS_SHA256_CTX, 1> = arr![Default::default(); 1];
        let mut digest_more_64_3: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
            0x68, 0x1d, 0x5b, 0x8f, 0xd1, 0xe7, 0x9b, 0x10, 0x17, 0x61, 0x52, 0x51, 0x35, 0x66,
            0xd9, 0xd2, 0xbc, 0x71, 0x31, 0xe0, 0x38, 0x70, 0xff, 0xb8, 0x4a, 0xe9, 0x63, 0xc6,
            0x3b, 0x1b, 0xde, 0x4f
        ];
        vosSha256Begin(stCtx.cast());
        vosSha256Hash(aucInput1, uiInlen1, stCtx.cast());
        vosSha256Hash(aucInput2, uiInlen2, stCtx.cast());
        vosSha256End(aucOutput.cast(), SHA256_DIGEST_SIZE!(), stCtx.cast());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_more_64_3.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    #[test]
    fn SDV_VosSha256Calc004() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput1: Ptr<u8> = SDV_TEXT_MORE_64!();
        let mut aucInput2: Ptr<u8> = SDV_TEXT_MORE_64!();
        let uiInlen1: u32 = c_strlen!(aucInput1) as u32;
        let uiInlen2: u32 = c_strlen!(aucInput2) as u32;
        let mut stCtx: Array<VOS_SHA256_CTX, 1> = arr![Default::default(); 1];
        let mut digest_more_128_4: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
            0x70, 0xde, 0x62, 0x9f, 0x46, 0xa1, 0xfc, 0x73, 0xf5, 0x25, 0x14, 0x72, 0x96, 0x06,
            0xb1, 0x4b, 0xf4, 0x1d, 0x65, 0x35, 0xc4, 0x1f, 0xe6, 0x93, 0x94, 0x4f, 0x67, 0x3d,
            0x0f, 0xfb, 0x78, 0x0e
        ];
        vosSha256Begin(stCtx.cast());
        vosSha256Hash(aucInput1, uiInlen1, stCtx.cast());
        vosSha256Hash(aucInput2, uiInlen2, stCtx.cast());
        vosSha256End(aucOutput.cast(), SHA256_DIGEST_SIZE!(), stCtx.cast());
        assert_eq!(
            Test_DigestCmp(
                aucOutput.cast(),
                digest_more_128_4.cast(),
                SHA256_DIGEST_SIZE!()
            ),
            0
        );
        test_no_memory_leak!();
    }

    // #[test]
    // fn SDV_VosSha256Calc005() {
    //     let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
    //     let mut puc_input1: Ptr<u8> = c_malloc!(0x40000000);
    //     if puc_input1 == NULL!() {
    //         assert_eq!(puc_input1, NULL!());
    //         return;
    //     }
    //     let mut i = 0;
    //     while i < 0x40000000 {
    //         puc_input1[i] = b'a';
    //         i += 1;
    //     }
    //     let uiInlen1: u32 = 0x40000000;
    //     let mut stCtx: Array<VOS_SHA256_CTX, 1> = arr![Default::default(); 1];
    //     let mut digest_0x40000000_5: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![
    //         0x95, 0xdf, 0x3e, 0xa6, 0x1d, 0xb5, 0x57, 0xb2, 0x2c, 0x1a, 0xbf, 0x60, 0x96, 0x45,
    //         0xc3, 0x42, 0x3b, 0xf8, 0x37, 0x74, 0xc2, 0x2c, 0x75, 0xe3, 0xc6, 0x37, 0xf8, 0xcb,
    //         0x7f, 0xc3, 0x3f, 0xd8
    //     ];
    //     vosSha256Begin(stCtx.cast());
    //     vosSha256Hash(puc_input1, uiInlen1, stCtx.cast());
    //     vosSha256Hash(puc_input1, uiInlen1, stCtx.cast());
    //     vosSha256End(aucOutput.cast(), SHA256_DIGEST_SIZE!(), stCtx.cast());
    //     c_free!(puc_input1);
    //     assert_eq!(
    //         Test_DigestCmp(
    //             aucOutput.cast(),
    //             digest_0x40000000_5.cast(),
    //             SHA256_DIGEST_SIZE!()
    //         ),
    //         0
    //     );
    //     test_no_memory_leak!();
    // }

    #[test]
    fn SDV_VosSha256Calc006() {
        let mut aucOutput: Array<u8, { SHA256_DIGEST_SIZE!() }> = arr![0; SHA256_DIGEST_SIZE!()];
        let mut aucInput: Ptr<u8> = SDV_TEXT_MORE_64!();
        let uiInlen: u32 = c_strlen!(aucInput) as u32;
        let mut stCtx: Array<VOS_SHA256_CTX, 1> = arr![Default::default(); 1];
        let mut ui_ret = 0;
        let mut i = 16;
        vosSha256Begin(stCtx.cast());
        vosSha256Hash(aucInput, uiInlen, stCtx.cast());
        vosSha256End(aucOutput.cast(), 16, stCtx.cast());
        while i < SHA256_DIGEST_SIZE!() {
            if aucOutput[i] != 0 {
                ui_ret = 1;
                break;
            }
            i += 1;
        }
        assert_eq!(ui_ret, 0);
        test_no_memory_leak!();
    }
}

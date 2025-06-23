pub mod md5_c;

use md5_c::*;

use crate::translation_utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testVOS_MD5Init() {
        let mut content: MD5_CTX = Default::default();
        VOS_MD5Init(NULL!());
        VOS_MD5Init(c_ref!(content));
        assert_eq!(content.aulState[0], 0x67452301);
        assert_eq!(content.aulState[1], 0xefcdab89);
        assert_eq!(content.aulState[2], 0x98badcfe);
        assert_eq!(content.aulState[3], 0x10325476);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Update() {
        let mut content: MD5_CTX = Default::default();
        let mut s: Ptr<u8> = cstr!("abcd");
        VOS_MD5Update(NULL!(), s, 4);
        VOS_MD5Update(c_ref!(content), NULL!(), 0);
        VOS_MD5Update(c_ref!(content), s, 0);
        c_memset_s!(
            c_ref!(content),
            c_sizeof!(MD5_CTX),
            0,
            c_sizeof!(MD5_CTX)
        );
        VOS_MD5Update(c_ref!(content), s, 4);
        assert_eq!(content.aulCount[0], 32);
        assert_eq!(c_strncmp!(content.aucBuffer, s, 4), 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5FinalEx() {
        let mut content: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        VOS_MD5FinalEx(NULL!(), 16, c_ref!(content));
        VOS_MD5FinalEx(digest.cast(), 10, c_ref!(content));
        VOS_MD5FinalEx(digest.cast(), 16, NULL!());
        c_memset_s!(
            c_ref!(content),
            c_sizeof!(MD5_CTX),
            0,
            c_sizeof!(MD5_CTX)
        );
        VOS_MD5FinalEx(digest.cast(), 20, c_ref!(content));
        c_memset_s!(
            c_ref!(content),
            c_sizeof!(MD5_CTX),
            0,
            c_sizeof!(MD5_CTX)
        );
        VOS_MD5FinalEx(digest.cast(), 16, c_ref!(content));
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5CalcEx() {
        let mut output: Array<u8, 20> = arr![0; 20];
        let mut resultCompare: Array<u8, 16> = arr![
            0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
            0x42, 0x7e
        ];
        VOS_MD5CalcEx(NULL!(), 0, NULL!(), 0);
        VOS_MD5CalcEx(output.cast(), 10, NULL!(), 0);
        VOS_MD5CalcEx(output.cast(), 20, NULL!(), 0);
        assert_eq!(c_strcmp!(output, resultCompare), 0);
        VOS_MD5CalcEx(output.cast(), 16, NULL!(), 0);
        assert_eq!(c_strcmp!(output, resultCompare), 0);
        test_no_memory_leak!();
    }

    macro_rules! TEST_MD5_A_INIT {
        () => {
            0x67452301
        };
    }
    macro_rules! TEXT_LESS_56 {
        () => {
            bstr!(b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc\0")
        };
    }
    macro_rules! TEXT_EQUAL_56 {
        () => {
            bstr!(b"12345678901234567890123456789012345678901234567890123456\0")
        };
    }
    macro_rules! TEXT_LESS_64 { () => { bstr!(b"\xe5\x95\x8a\xe6\xb3\xa2\xe6\xac\xa1\xe7\x9a\x84\xe9\xa2\x9d\xe4\xbd\x9b\xe6\xad\x8c\xe9\xa2\x9d\xe5\xb7\xb2\xe7\xba\xa7\xe5\x8f\xaf\xe4\xba\x86\xe8\x8e\xab\xe5\x91\xa2\xe5\x93\xa6\xe7\xa0\xb4\xe5\x99\xa8\xe9\x88\xa4\xe6\x96\xaf\xe7\x89\xb9\xe5\x94\x94\0")} }
    macro_rules! TEXT_EQUAL_64 {
        () => {
            bstr!(b"./,!@#$%^&*(){}[\"``~..\"/.;':<>?';:,./:>?:*&^$##%&^&^$!@;.!@#$%^&\0")
        };
    }
    macro_rules! TEXT_MORE_64 { () => { bstr!(b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%\0") } }
    macro_rules! TEXT_EQUAL_128 { () => { bstr!(b"a123b\"lmnop\xe8\x9b\x8b7345&^$##6\xe9\xa5\xad4^&5*&^$##\xe7\xb3\x96%678o12`~..3pq&^$#rst#%\"ef&5*&^$#896\xe7\x89\x9b7345&^$##6\xe7\xbe\x8a4^&5*&^$##%678o\xe7\x8c\xaa12`~..3pq&^$#rst#%\0") } }
    macro_rules! DIGEST_LEN {
        () => {
            16
        };
    }

    fn Test_MD5Calc(mut input: Ptr<u8>, mut resultCompare: Ptr<u8>) {
        let mut output: Array<u8, 16> = arr![0; 16];
        let mut inLen: usize = if input == NULL!() {
            0
        } else {
            c_strlen!(input)
        };
        VOS_MD5Calc(output.cast(), input, inLen.cast());
        assert_eq!(c_strncmp!(output, resultCompare, 16), 0);
    }

    #[test]
    fn testVOS_MD5Calc001() {
        let mut resultCompare: Array<u8, 16> = arr![
            0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
            0x42, 0x7e
        ];
        Test_MD5Calc(NULL!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc002() {
        let mut resultCompare: Array<u8, 16> = arr![
            0x0d, 0x7a, 0xe0, 0x56, 0xb2, 0xf0, 0x15, 0xcd, 0x7d, 0xc6, 0x74, 0x94, 0xef, 0xd6,
            0x58, 0xf1
        ];
        Test_MD5Calc(TEXT_LESS_56!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc003() {
        let mut resultCompare: Array<u8, 16> = arr![
            0x49, 0xf1, 0x93, 0xad, 0xce, 0x17, 0x84, 0x90, 0xe3, 0x4d, 0x1b, 0x3a, 0x4e, 0xc0,
            0x06, 0x4c
        ];
        Test_MD5Calc(TEXT_EQUAL_56!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc004() {
        let mut resultCompare: Array<u8, 16> = arr![
            0x57, 0x7b, 0xe3, 0xed, 0x8e, 0x9f, 0xa4, 0x87, 0xbd, 0x81, 0x2d, 0xd8, 0x1c, 0x20,
            0x3a, 0x21
        ];
        Test_MD5Calc(TEXT_LESS_64!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc005() {
        let mut resultCompare: Array<u8, 16> = arr![
            0xeb, 0x2b, 0x3f, 0xb8, 0xbc, 0x26, 0x07, 0x35, 0x1b, 0x37, 0xfd, 0x83, 0x03, 0x5b,
            0xe3, 0xb5
        ];
        Test_MD5Calc(TEXT_EQUAL_64!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc006() {
        let mut resultCompare: Array<u8, 16> = arr![
            0xcd, 0x29, 0x93, 0x8c, 0xa4, 0x8b, 0x59, 0x17, 0xe0, 0x61, 0x94, 0x98, 0xae, 0x9c,
            0x53, 0x49
        ];
        Test_MD5Calc(TEXT_MORE_64!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc007() {
        let mut resultCompare: Array<u8, 16> = arr![
            0xd7, 0x1c, 0x48, 0x3e, 0xc6, 0x00, 0xea, 0x7c, 0x0b, 0x7e, 0xd5, 0x25, 0x45, 0x7d,
            0xe9, 0xe8
        ];
        Test_MD5Calc(TEXT_EQUAL_128!(), resultCompare.cast());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5Calc008() {
        let mut inLen: usize = 0x20000000;
        let mut input: Ptr<u8> = c_malloc!(inLen);
        let mut output: Array<u8, 16> = arr![0; 16];
        let mut resultCompare: Array<u8, 16> = arr![
            0x02, 0xe4, 0xcd, 0x84, 0x8b, 0xee, 0x68, 0xa0, 0x08, 0x7d, 0xf0, 0x6b, 0xb6, 0xe7,
            0xc8, 0xa2
        ];
        for i in 0..inLen {
            input[i] = (i % 10) as u8 + 'a' as u8;
        }
        VOS_MD5Calc(output.cast(), input, inLen.cast());
        c_free!(input);
        assert_eq!(c_strncmp!(output, resultCompare, 16), 0);
        test_no_memory_leak!();
    }

    fn Test_MD5Update(mut input: Ptr<u8>) {
        let mut inputLen: usize = c_strlen!(input);
        let mut context: MD5_CTX = Default::default();
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        if context.aulState[0] == TEST_MD5_A_INIT!() {
            assert!(false);
        }
    }

    #[test]
    fn testVOS_MD5009() {
        let mut input: Ptr<u8> = TEXT_LESS_56!();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut context: MD5_CTX = Default::default();
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        if context.aulState[0] != TEST_MD5_A_INIT!() {
            assert!(false);
        }
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5010() {
        let mut input: Ptr<u8> = TEXT_LESS_64!();
        let mut context: MD5_CTX = Default::default();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        assert!(context.aulState[0] == TEST_MD5_A_INIT!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5011() {
        Test_MD5Update(TEXT_EQUAL_64!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5012() {
        Test_MD5Update(TEXT_MORE_64!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5013() {
        Test_MD5Update(TEXT_EQUAL_128!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5014() {
        let mut input: Ptr<u8> = TEXT_LESS_56!();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        if context.aulState[0] != TEST_MD5_A_INIT!() {
            assert!(false);
        }
        VOS_MD5Final(digest.cast(), c_ref!(context));
        if c_strlen!(digest) <= 0 {
            assert!(false);
        }
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5015() {
        let mut input: Ptr<u8> = TEXT_LESS_64!();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        assert!(context.aulState[0] == TEST_MD5_A_INIT!());
        VOS_MD5Final(digest.cast(), c_ref!(context));
        if c_strlen!(digest) <= 0 {
            assert!(false);
        }
        test_no_memory_leak!();
    }

    fn Test_MD5Final(mut input: Ptr<u8>) {
        let mut inputLen: usize = c_strlen!(input);
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        VOS_MD5Init(c_ref!(context));
        if context.aulCount[0] != 0 && context.aulCount[1] != 0 {
            assert!(false);
        }
        VOS_MD5Update(c_ref!(context), input, inputLen.cast());
        if context.aulState[0] == TEST_MD5_A_INIT!() {
            assert!(false);
        }
        VOS_MD5Final(digest.cast(), c_ref!(context));
        if c_strlen!(digest) <= 0 {
            assert!(false);
        }
    }

    #[test]
    fn testVOS_MD5016() {
        Test_MD5Final(TEXT_EQUAL_64!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5017() {
        Test_MD5Final(TEXT_MORE_64!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5018() {
        Test_MD5Final(TEXT_EQUAL_128!());
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5019() {
        let mut input: Ptr<u8> = TEXT_EQUAL_128!();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        c_memset_s!(
            c_ref!(context),
            c_sizeof!(MD5_CTX),
            0,
            c_sizeof!(MD5_CTX)
        );
        VOS_MD5Init(NULL!());
        assert!(context.aulState[0] == 0);
        VOS_MD5Update(NULL!(), input, inputLen);
        assert!(context.aulCount[0] == 0);
        VOS_MD5Final(digest.cast(), NULL!());
        assert!(c_strncmp!(digest, cstr!(""), 16) == 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5020() {
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 20> = arr![0; 20];
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), NULL!(), 10);
        assert!(context.aulCount[0] == 0);
        VOS_MD5Final(digest.cast(), c_ref!(context));
        if c_strlen!(digest) == 0 {
            assert!(false);
        }
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5021() {
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 16> = arr![0; 16];
        let mut input: Ptr<u8> = TEXT_LESS_56!();
        let mut inLen: u32 = c_strlen!(input) as u32;
        let mut resultCompare: Array<u8, 16> = arr![
            0x0d, 0x7a, 0xe0, 0x56, 0xb2, 0xf0, 0x15, 0xcd, 0x7d, 0xc6, 0x74, 0x94, 0xef, 0xd6,
            0x58, 0xf1
        ];
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inLen);
        VOS_MD5FinalEx(digest.cast(), 16, c_ref!(context));
        assert!(c_strncmp!(digest, resultCompare, 16) == 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5022() {
        let mut context: MD5_CTX = Default::default();
        let mut digest: Array<u8, 15> = arr![0; 15];
        let mut input: Ptr<u8> = TEXT_LESS_56!();
        let mut inLen: u32 = c_strlen!(input) as u32;
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inLen);
        VOS_MD5FinalEx(digest.cast(), 15, c_ref!(context));
        assert!(c_strncmp!(digest, cstr!(""), 16) == 0);
        test_no_memory_leak!();
    }

    fn Test_MD5MultipleUpdate(mut inputHead: Ptr<u8>, mut inputTail: Ptr<u8>, mut inputCombine: Ptr<u8>) {
        let mut separateOutput: Array<u8, 16> = arr![0; 16];
        let mut combineOutput: Array<u8, 16> = arr![0; 16];
        let mut context: MD5_CTX = Default::default();
        let mut inputHeadLen: u32 = c_strlen!(inputHead) as u32;
        let mut inputTailLen: u32 = c_strlen!(inputTail) as u32;
        let mut inputCombineLen: u32 = c_strlen!(inputCombine) as u32;
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), inputCombine, inputCombineLen);
        VOS_MD5Final(combineOutput.cast(), c_ref!(context));
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), inputHead, inputHeadLen);
        VOS_MD5Update(c_ref!(context), inputTail, inputTailLen);
        VOS_MD5Final(separateOutput.cast(), c_ref!(context));
        assert!(c_strncmp!(combineOutput, separateOutput, 16) == 0);
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_001() {
        Test_MD5MultipleUpdate(
            cstr!("abcdefghijklmnopqrstuvw"),
            cstr!("xyzabcdefghijklmnopqrstuvwxyzabcdefghijk"),
            cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijk"),
        );
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_002() {
        Test_MD5MultipleUpdate(
            cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvw"),
            cstr!("xyzabcdefghijkl"),
            cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl"),
        );
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_003() {
        Test_MD5MultipleUpdate(
            cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"),
            cstr!("abcdefghijklm"),
            cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklm"),
        );
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_004() {
        Test_MD5MultipleUpdate(cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"), cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"), cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"));
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_005() {
        Test_MD5MultipleUpdate(cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"), cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"), cstr!("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"));
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_006() {
        Test_MD5MultipleUpdate(cstr!("a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖"), cstr!("a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*("), cstr!("a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*("));
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_007() {
        let mut context: MD5_CTX = Default::default();
        let mut inputHeadLen: u32 = 0x20000000;
        let mut inputHead: Ptr<u8> = c_malloc!(inputHeadLen);
        let mut inputTail: Ptr<u8> = cstr!("12345678910");
        let mut inputTailLen: u32 = c_strlen!(inputTail) as u32;
        let mut output: Array<u8, 20> = arr![0; 20];
        let mut outputResult: Array<u8, 16> = arr![
            0xfc, 0x73, 0xcb, 0x71, 0x4d, 0xf6, 0x8d, 0x82, 0x6a, 0xc3, 0x3d, 0x66, 0x49, 0x36,
            0xc0, 0x72
        ];
        for i in 0..inputHeadLen {
            inputHead[i] = (i % 10) as u8 + 'a' as u8;
        }
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), inputHead, inputHeadLen);
        VOS_MD5Update(c_ref!(context), inputTail, inputTailLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        c_free!(inputHead);
        assert!(c_strncmp!(output, outputResult, 16) == 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_008() {
        let mut context: MD5_CTX = Default::default();
        VOS_MD5Init(c_ref!(context));
        context.aulCount[0] = 0xffffffff;
        context.aulCount[1] = 0xffffffff;
        VOS_MD5Update(c_ref!(context), cstr!("abcdefg"), 7);
        assert!(c_strncmp!(context.aucBuffer, cstr!(""), 16) == 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_MUTIPLE_UPDATE_009() {
        let mut inputHead: Ptr<u8> =
            cstr!("a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖");
        let mut inputTail: Ptr<u8> =
            cstr!("a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*(");
        let mut output1: Array<u8, 17> = arr![0; 17];
        let mut output2: Array<u8, 17> = arr![0; 17];
        let mut context: MD5_CTX = Default::default();
        let mut inputHeadLen: u32 = c_strlen!(inputHead) as u32;
        let mut inputTailLen: u32 = c_strlen!(inputTail) as u32;
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), inputHead, inputHeadLen);
        VOS_MD5Update(c_ref!(context), inputTail, inputTailLen);
        VOS_MD5Final(output1.cast(), c_ref!(context));
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), inputTail, inputTailLen);
        VOS_MD5Update(c_ref!(context), inputHead, inputHeadLen);
        VOS_MD5Final(output2.cast(), c_ref!(context));
        assert!(c_strncmp!(output1, output2, 16) != 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_API_CALL001() {
        let mut input: Ptr<u8> = TEXT_EQUAL_56!();
        let mut output: Array<u8, 16> = arr![0; 16];
        let mut context: MD5_CTX = Default::default();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut resultCompare: Array<u8, 16> = arr![
            0x49, 0xf1, 0x93, 0xad, 0xce, 0x17, 0x84, 0x90, 0xe3, 0x4d, 0x1b, 0x3a, 0x4e, 0xc0,
            0x06, 0x4c
        ];

        VOS_MD5Init(c_ref!(context));
        VOS_MD5Final(output.cast(), c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        assert!(c_strncmp!(output, resultCompare, 16) != 0);

        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        VOS_MD5Init(c_ref!(context));
        assert!(c_strncmp!(output, resultCompare, 16) != 0);

        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Final(output.cast(), c_ref!(context));
        assert!(c_strncmp!(output, resultCompare, 16) != 0);

        VOS_MD5Final(output.cast(), c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Init(c_ref!(context));
        assert!(c_strncmp!(output, resultCompare, 16) != 0);

        VOS_MD5Final(output.cast(), c_ref!(context));
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        assert!(c_strncmp!(output, resultCompare, 16) != 0);

        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        assert!(c_strncmp!(output, resultCompare, 16) == 0);
        test_no_memory_leak!();
    }

    #[test]
    fn testVOS_MD5_API_CALL002() {
        let mut input: Ptr<u8> = TEXT_EQUAL_56!();
        let mut output: Array<u8, 17> = arr![0; 17];
        let mut context: MD5_CTX = Default::default();
        let mut inputLen: u32 = c_strlen!(input) as u32;
        let mut outputResult: Array<u8, 16> = arr![
            0x49, 0xf1, 0x93, 0xad, 0xce, 0x17, 0x84, 0x90, 0xe3, 0x4d, 0x1b, 0x3a, 0x4e, 0xc0,
            0x06, 0x4c
        ];

        VOS_MD5Init(c_ref!(context));
        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        assert!(c_strncmp!(output, outputResult, 16) == 0);

        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        assert!(c_strncmp!(output, outputResult, 16) != 0);

        VOS_MD5Init(c_ref!(context));
        VOS_MD5Update(c_ref!(context), input, inputLen);
        VOS_MD5Final(output.cast(), c_ref!(context));
        VOS_MD5Final(output.cast(), c_ref!(context));
        assert!(c_strncmp!(output, outputResult, 16) != 0);
        test_no_memory_leak!();
    }
}

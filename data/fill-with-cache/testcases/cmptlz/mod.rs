pub mod cmptlz_c;

use cmptlz_c::*;

use crate::translation_utils::*;
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! getLine { () => { c__line__!() }; }

    static mut g_logBuf: Array<u8, 256> = arr![0; 256];
    static mut g_logSize: usize = 0;

    fn testLogFunc(message: Ptr<u8>, size: usize) {
    unsafe {
        c_snprintf_s!(g_logBuf, 256, 256, cstr!("{}"), message);
        g_logSize = size;
    }}

    #[test]
    fn UT_CmptlzLogRegister_FUNC_001() {
        unsafe {
            CmptlzLogRegister(func!(testLogFunc));
            let mut expLog = arr![0; 100];
            CMPTLZ_LOG!(-1isize as usize, cstr!("something"));
            c_snprintf_s!(expLog, 100, 100,
                cstr!("\n[Cmptlz-Log] Func=UT_CmptlzLogRegister_FUNC_001, Line={}, Error={}\nsomething"), getLine!() - 2,
                -1isize as usize);
            println!("expLog: {}", expLog.cast::<Ptr<u8>>().to_string());
            println!("g_logBuf: {}", g_logBuf.cast::<Ptr<u8>>().to_string());
            assert_eq!(c_strncmp!(g_logBuf, expLog, g_logSize), 0);
            assert_eq!(g_logSize, c_strlen!(expLog) + 1);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CmptHeadWrite_FUNC_001() {
        let mut prot: Array<u8, 5> = arr![0; 5];
        let mut protSize = 5;
        let mut encCtx: CmptLzEncCtx = Default::default();
        encCtx.dicSize = 1;
        encCtx.litCtx = 3;
        encCtx.litPos = 0;
        encCtx.posBits = 2;
        let ret = CmptHeadWrite(c_ref!(encCtx), prot.cast(), c_ref!(protSize));
        assert_eq!(ret, 0);
        assert_eq!(prot[0], 93);
        assert_eq!(prot[1], 1);
        assert_eq!(prot[2], 0);
        assert_eq!(prot[3], 0);
        assert_eq!(prot[4], 0);
        let ret = CmptHeadWrite(c_ref!(encCtx), NULL!(), c_ref!(protSize));
        assert_eq!(ret, CMPT_ENC_ERROR_HEAD!());
        protSize = 3;
        let ret = CmptHeadWrite(c_ref!(encCtx), prot.cast(), c_ref!(protSize));
        assert_eq!(ret, CMPT_ENC_ERROR_HEAD!());
        test_no_memory_leak!();
    }

    fn TestLogFunc(mut message: Ptr<u8>, size: usize) {
        println!("LogError : {} Length : {}", message, size);
    } 

    fn testAllocCmptLz(mut enMemType: i32, mut size: usize) -> VoidPtr {
        if size == 0 {
            return NULL!();
        }
        return c_malloc!(size);
    }

    fn testFreeCmptLz(mut enMemType: i32, mut address: VoidPtr) {
        if address == NULL!() {
            return;
        }
        c_free!(address);
    }

    static mut g_public_cmptlz_alloc: CmptLzMemHook = CmptLzMemHook {
        CmptLzAlloc: func!(testAllocCmptLz),
        CmptLzFree: func!(testFreeCmptLz),
    };

    fn test_cmptlz_enc_default(mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut src: Ptr<u8>, mut srcLen: usize) -> i32 {
    unsafe {
        let mut param: CmptlzCompParam = Default::default();
        param.level = 5;
        param.litCtx = 3;
        param.litPos = 0;
        param.posBits = 2;
        param.fastBytes = 32;
        param.numThreads = 1;
        param.dictSize = 16 * 1024 * 1024;
        param.memHook = c_ref!(g_public_cmptlz_alloc);
        let mut outProps: Array<u8, 5> = arr![0; 5];
        param.protData = outProps.cast();
        param.protSize = 5;
        return CmptlzCompress(src.cast(), srcLen, dest.cast(), destLen, c_ref!(param));
    }}

    fn testOpenFile(mut filePath: Ptr<u8>, mut srcPtr: Ptr<Ptr<u8>>, mut srcSizePtr: Ptr<usize>) {
    unsafe {
        let mut fp = c_fopen!(filePath, cstr!("rb"));
        assert!(fp != NULL!());
        c_fseek!(fp, 0, SEEK_END!());
        let mut srcSize = c_ftell!(fp);
        c_fseek!(fp, 0, SEEK_SET!());
        let mut src: Ptr<u8> = c_malloc!(srcSize + 1);
        src[srcSize] = 0;
        let mut readCount: usize = c_fread!(src, 1, srcSize, fp);
        assert_eq!(readCount, srcSize as usize);
        *srcPtr = src;
        *srcSizePtr = srcSize as usize;
        assert_eq!(c_fclose!(fp), 0);
    }}

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_001() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            let fileEncPath = cstr!("data/test_file/corpus/Artificial/alphabet.txt");
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            println!("encSrc: {}", encSrc.cast::<usize>());
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_002() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc = cstr!("ABCDABCDABCDABCDABCABCABCABCABCABCABCASJKGFWAUWABCACABOEFHVCOABCABCABCABCABCABCABCASJKGFWAUWABCACABOEFHVCOABCABCABCABCABCABCABCASJKGFWAUWABCACABOEFHVCO");
            let mut encLen = c_strlen!(encSrc);
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_003() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc1 = cstr!("1");
            let mut encLen1 = c_strlen!(encSrc1);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc1, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_004() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc2 = cstr!("1A");
            let mut encLen1 = c_strlen!(encSrc2);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc2, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_005() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc3 = cstr!("1ABCGDMN");
            let mut encLen1 = c_strlen!(encSrc3);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc3, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_006() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc4 = cstr!("1ABCDABCD");
            let mut encLen1 = c_strlen!(encSrc4);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc4, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_007() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc5 = cstr!("1ABCDABCDA");
            let mut encLen1 = c_strlen!(encSrc5);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc5, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_008() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc6 = cstr!("1ABCDABCDAB");
            let mut encLen1 = c_strlen!(encSrc6);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc6, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_009() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc7 = cstr!("1ABCDABCD1ABD1");
            let mut encLen1 = c_strlen!(encSrc7);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc7, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_010() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc8 = cstr!("1ABCDABCD1ABDABBD");
            let mut encLen1 = c_strlen!(encSrc8);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc8, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_011() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc9 = cstr!("1ABCDABCD1ABDABCD1BC");
            let mut encLen1 = c_strlen!(encSrc9);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc9, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_012() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc10 = cstr!("1ABCDABCDABEFGB");
            let mut encLen1 = c_strlen!(encSrc10);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc10, encLen1);
            c_free!(destination);
            destination = NULL!();
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen1);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen1 as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen1 as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_013() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let mut encSrc11 = cstr!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
            let mut encLen = c_strlen!(encSrc11);
            let mut destinationLen = 1 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc11, encLen);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_014() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/corpus/Artificial/random.txt");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_015() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/nengyuan/Optimizer_pwr.bin");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_016() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/nengyuan/sd5012_demo_sta.bin");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_017() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/shutong/APP_part");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            println!("encLen: {}", encLen);
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_018() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/shutong/gpsfu16grb_pwrepldP.bin");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }
    
    #[test]
    fn UT_CMPTLZ_ENCODE_SYMBOL_FUNC_019() {
        unsafe {
            CmptlzLogRegister(func!(TestLogFunc));
            let fileEncPath = cstr!("data/test_file/shutong/appCompress_Linux");
            let mut encLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encLen));
            let mut destinationLen = 10 * 1024 * 1024;
            let mut destination: Ptr<u8> = c_malloc!(destinationLen);
            let res = test_cmptlz_enc_default(destination, c_ref!(destinationLen), encSrc, encLen);
            c_free!(encSrc);
            c_free!(destination);
            assert_eq!(res, 0);
            println!("压缩前占用了 {} 的内存空间", encLen);
            println!("压缩后占用了 {} 的内存空间", destinationLen);
            println!("压缩前/压缩后 为 {}", (encLen as f64) / (destinationLen as f64));
            println!("压缩后/压缩前 为 % {}", (destinationLen as f64) / (encLen as f64) * 100.0);
            test_no_memory_leak!();
        }
    }

    fn testStringCmp(str1: Ptr<u8>, str2: Ptr<u8>, len: usize) {
        for ind in 0..len {
            assert_eq!(str1[ind], str2[ind]);
        }
    }


    // void testLog(const char *message, size_t size)
    // {
    //     (void)size;
    //     printf("%s\n", message);
    // }

    fn testLog(message: Ptr<u8>, size: usize) {
        println!("{}", message.to_string());
    }

    fn test_cmptlz_enc_and_cmptlz_dec(mut encSrc: Ptr<u8>, mut encSrcLen: usize) {
        unsafe {
        if encSrcLen == 0 {
            return;
        }
        CmptlzLogRegister(func!(testLog));
        let mut res = -1;
        let mut encDestLen = encSrcLen * 2 + 100;
        let mut encDest = c_malloc!(encDestLen);
        let mut decDestLen = encSrcLen;
        let mut decDest = c_malloc!(decDestLen);
        res = test_cmptlz_enc_default(encDest, c_ref!(encDestLen), encSrc, encSrcLen);
        assert_eq!(res, 0);
        let mut param: CmptlzDecParam = Default::default();
        param.memHook = c_ref!(g_public_cmptlz_alloc);
        let mut protData: Array<u8, 5> = arr![93, 0, 0, 0, 0];
        param.protData = protData.cast();
        param.protSize = 5;
        res = CmptlzDecompress(encDest, encDestLen, decDest, c_ref!(decDestLen), c_ref!(param));
        assert_eq!(res, 0);
        assert_eq!(decDestLen, encSrcLen);
        res = c_memcmp!(decDest, encSrc, encSrcLen);
        assert_eq!(res, 0);
        c_free!(decDest);
        c_free!(encDest);
    }}

    fn test_string_cmptlz_enc_cmptlz_dec(mut encSrc10: Ptr<u8>) {
        unsafe {
            let mut encSrcLen = c_strlen!(encSrc10);
            println!("压缩前的数据长度为{}", encSrcLen);
            println!("压缩前的数据为{}", encSrc10.to_string());
            test_cmptlz_enc_and_cmptlz_dec(encSrc10, encSrcLen);
        }
    }

    fn test_file_cmptlz_enc_cmptlz_dec(mut fileEncPath: Ptr<u8>) {
        unsafe {
            let mut encSrcLen = 0;
            let mut encSrc: Ptr<u8> = NULL!();
            testOpenFile(fileEncPath, c_ref!(encSrc), c_ref!(encSrcLen));
            test_cmptlz_enc_and_cmptlz_dec(encSrc, encSrcLen);
            c_free!(encSrc);
        }
    }
    
    #[test]
    fn UT_CMPTLZ_ENCODE_AND_DECODE_FUNC_001() {
    unsafe {
        let mut enc0 = cstr!("srgcswwwT0Op");
        let mut enc1 = cstr!("1ABCDABCDABEFGB");
        let mut enc2 = cstr!("1ABCDEFGHJKLIADOM");
        let mut enc3 = cstr!("ABCDABC");
        let mut enc4 = cstr!("11ABCDEFGHIJKLABCDE");
        let mut enc5 = cstr!("ABCDABCDHBCDHB");
        let mut enc6 = cstr!("1ABCD1ABCD1A2CD1A23D1A2");
        let mut enc7 = cstr!("1ABCD1ABCD1AXYPQA");
        let mut enc8 = cstr!("1ABCD1ABCD1AXYPQA0xyz");
        let mut enc9 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5ABCDE6ABCDEFG7ABCD8ABCD9ABCDE10ABCDEF11ABCD");
        let mut enc10 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5ABCDE6ABCDEFG7ABCDEFGH8ABCD9GH8AB10DEFGHxy8AB10D");
        let mut enc11 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5ABCDE6ABCDEFG7ABCDEFGH8ABCD9");
        let mut enc12 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5A");
        let mut enc13 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5A1ABCD2ABCDE3ABCDEF4ABCD5A1ABCD2ABCDE3ABCDEF4ABCD5A1ABCD2ABCDE3ABCDEF4ABCD5A1ABCD2ABCDE3ABCDEF4ABCD5A1ABCD2ABCDE3ABCDEF4ABCD5A");
        let mut enc14 = cstr!("1ABCDEFG2ABCDEFG");
        let mut enc15 = cstr!("1ABCD2ABCDE3ABCDEF4ABCD5ABCDE6ABCDEFG7ABCDEFGH8ABCD9GH8AB10DEFGHxy8AB10D");
        let mut enc16 = cstr!("sUskR6JBP4B1W9zZj3Pr!509o21347645ABCDEFGHsUskR6JBP4B1W9zZj3Pr!509o21347645ABCDEFGH");
        let mut enc17 = cstr!("sUksUR6JBP4B1W9zZj3Pr!509o");
        let mut enc18 = cstr!("sUsUkR6JBP4B1W9zZj3Pr!509o");
        let mut enc19 = cstr!("68123sUsUkR6JBP4B1W9zZj3Pr!509o");
        let mut enc20 = cstr!("sUsUkR6JBP4B1W9zZj3Pr!509o");
        let mut enc21 = cstr!("srgcswwwT0Op");
        let mut enc22 = cstr!(" zBf9QD9zEkloaoF5Jx8RttdsKurdAJHN Y!53NHueGUvqFpdqj5RP2IpHy20621r2VYtdFJnLIq4BufzvxtlwNGqRMyPBXi2h4GibFpGzZ08tu7bPSjYyUjdDHN8PFkpkZBEc8ahCISnXfRl565oLmNxiKnN8aPAan jJULzfT wYZlU2FlKs96W97g 9OV P Zi2F4Vb9s0 OYxSucvnD4e8qjv6bhbrZ7HQQaed vp9wLDRLvniVYEYPXb7A iD9iY1vDICre5mKAPEFJF5QfuySzBE6tmbtSMATGOyW85Hl!VwCdYbqZYL3SeVB2KE5igLbK0PXkbLwm7MvCFkFzd3cBrTEc1YLnxWyIHlFZqKZrhkOahFIJiSaeHK!hwzUrrU6l9e1eEVEvWuA66GuqcUkjCbbo!sMBonliQub cVxvK!W8o63eul0qwPQk3oj589TldZ2ITt5maNSvH80wa7xRWwUH4RTT6BSy6XUAmyuHAhVoJaez9FkiiG t!VrmjL36aGdrgF!8lSMScL4n421Ep JniZGkj5jp5ABEc3!7c98gbE!Zx2IE7jjwckaiI7!Y H8gpYAm!AvnUdzvjAoLSMaPMZB9wzdclrLz67O0rb36joawku5Y eEudRNfxix2rMnldmKGz7PaSofw8b9HBSHXKa5ZG3dViidytqgNfzVY5ungb5ZnDJwQ03XSkp4UjdwR6N3xZ2gB6Oizf3wMt2QfeLK8bRsu!hthVuMdnN1sZ pEYbbX9vrItbrJmagjCao!maah!eIrxJgy GmsX G03c7DfpVzSNpljfn6y!2utSKCQAGeyA814Seex4146pqP1YZBJPYAWXDJpFvr7Xwo3gnGhHG3SlVVq!yn5lNnaLSDqDQjIIGT m8LYZxmNokLdeMDaYklcK4IvFySnL174Fmy!bTLtgfz4QDKZpg3Lsy20pMEgFgXmtiNqbvUobld7vDBQAcQuNvdmOOhpiej6uEdyTFsQJ4ftdvo9iQUwejZmCTPH0Jk!4O5gCFeWw55xBp8xJ8eEAbtEaQ91 2uI!r0YknQ8nSRfHT3WdJum!1jx3scaj9et2ow52uWhpLyIV8UFlytXafXd9!xEz2pnv8hIaefxkxySWdoLKa2EL x0Ju2ueNaFKvrVTnPmKpgqVqgg0o7uA77YWrC1bUVdWxkqjoDOQeJUFqOlnP1ZE99ReCuncn2JTk6MLH57ib aWP2lIR!N8Ax!DGJ!99gXlK5 wtE2wb17nyu40!!7gFaGIohco!GnSfqTu!1bT9P6jdTxgTiUgPx5hwtc0aJcRMBrfP5mpAVC!X1jlC7bv2oMnxS689IM3ABI Qs4uB6MiCIhZjiItayemxgNcu5mTIAhbmwg7ZOHiv4SFxli!y6Y3FDW iifPsmkwgs39SzzEWw50uDCfERmuM1!hnd1Wu5BShSE8nm8u5vH1AEwbxsi!TuXybe0WsKTRLDZe8SklM4ATmcCas hldWlN6wctQ5z TYuQLiXcom!lrUOwjyNxpum8ECDbtoCygln3RRG7vLwrlysGhsEVxPm79BxcADM9hC ixh57DHOPz6i5iloYn1KmLyv9OuMlLfXJ1IQ2BrZRa6Ts6nKmhinziqf3zHehTUXCTBZtS6yRMCZQfZEfk0n8fux3L0GYUZdX!Fy5JScdKOjxUzpyt7qIrtADBlsLW2bC5U7o1wvWWWRa6hf2x8p0lBzZJySG3rMlk4t9hzmLBnePEiqWtbP6Z1sMmAb1gCVX8IDuhLByKXnBzmCSIiU5EEcQp tTDWnq!ttT tRElTHeM!q36Yqy2i6zI3bs50RNasnh5klQgz3OBS7UKvtlN9Qe T3wwxvvNNXZMeih9PHBJ1k4A09P I9gP76WA19sEYFTILhWn FYaXdMuDLxzUfdr5O8bjU q9fkyricPd7hFvlqrcKyWPw6WYlnw26egfJUcfdd1I133XCD!9o C2ohx88GO14a5zaD9q nhxNFkr57PM8vQEHlsvoqnhwpgrCpjraFlI9kTTdl2RV77YnbZ9YSm8S WcNzNLvYmnyY7!aMscBXlGBVl5jaSzorbzncAgfhbFHOPeEhUf6uTEHMSHuhT6CDbMq HzK0nhyt9yoA18MbLr I!XJTXdkpaA1AeW3!lZDxQ0!haz3U6jMJPdi6Nn!lfAugJfKYHQwr2KFZsFnMaEW3Z0N!GWXR f8xcwLrA3Qx53tuGLQ6i52ih1hKF4YHoR5r mY5SfLuZPyvf8VfhEVwMQ43sqZHRlnPFUrjqn9aexKP hfN1P 822PHv7A4CLMLKDTZwrteESLZfBr8CxRYcLJKzhh6KWapnTPNnRCl9cI4iGttpfsGNbqorPobVYsjTJYjzsUosXMBMYPKpG1I6RrdIWyVR29obihZXBoV9wln2fU8 qyqsq !40Vk9wpkRmd uWyucIFaLt2YVepmz3Lzzqjdgq UJ6ue7l!Dpmxusnqoxmp1IE7CWiAG69BdiiSsNJtDoK3CXUZkOo528qtlr62v9uB7aln3tfBem852ui7R1gwKn4T!KqA93oIl4Ef!MZa!iW2saVa9YbQpUuHrB5ndxE3lBomnSGBDTtl3V QMQxfCHgAQzbQM YjysnuOk9YPvlVAZwqSA!2atWbDomUFwpS9ujxFlE2VeZ7ma8CKPkt!igx6!sYMJMqUi5jnIqHXIRWMGyo0ed aBYGmBEOd FNlOXsMLJ4TowPVB3FifF6w26aR wpGSFQyWAfFN3NFF18jPYhEZw4oWFTAlvYLX0!E!K5xLwuxVAIzWKi6 P lLfMcJlLTaKdQt5PlRs9F2Sv2eo!bW4Ub9Bi400BeboQXGLyKdDh5TJsoEaoXplGwPLnFG5oywQpX9yUFcTnejT Mqal2sm6wNSjmQ5yYSD UcbS1ysvn JvYZdliP6zwlQ5uaN64g SYp!mFXW1oOi!gyUdMCabERk qRd NaG!TY7HklohDkY73JAdZojYo36BWWZQjj1s4pohqSdZpMQE1 QDdrHcveWT i2j03oPdBBHIL7ihf5R2420kW8N66X3oZgZ2HthNbO1i4afWBbF wrGihSxd93JCraTm9qAb6Bnt0QLmH!kM56jS7gcNMj5Bu35v7ZhQlY7gFtMj9On2TcpB3R7LyTt 2vD1hwzVPoSUtWpA47qaMMbLoniAkEAkqf 68tseoj!7Uv6oERf7llZIRqjr2uCEUTMm9Hk7SCy0YVaOfQbMvpT D0!2D0UZmcMkD1ZM9YFAsullLv95orUBb24ErGaDtEE!JcaFvr039wzVoJnNb42UGV3tsaxWyNWYZabN!J5KWz1D AeCYKnfC!O!PUnfMU TKoPGQHLX D4ayKfrHkstCsgiGFzNPc0b8cvbvePXPrFCZh4NwdzPCJyYLvtKRIJAPfi iCyf7FVJvyzxiTG9E3XFDbt3Xb4vZHFscVGACpdHrDtcXvUvPdKL4UCMAtYg3CqXsWhRosiUuUUZTdvnv1rksP0RshPQYiOZfcAi1bZL83uhzOhjMd6iPfdZEhYB32!m1YHxa092TWZ8ZSw5A Fg9xPZ6tyKmvXuz8KcF256v3MKEk9QZlksQnI8fkEyIzPncduNMYsBOmjG8DkfC4VIhum0cszh6ApBTKtYxOIQ5vd 16XFUQD XLmVamVpuuH3 bacBSKCUO9vZQP8PVCKsLaI5ry7H3ZHM8HwdIBDPphG9Gibst pz821cqgx1Jy4eujV!O0ZUpCL65EiFl3hxPvwkBb9jglqmoDBwfsLkceA2yvBGC3mTpLj96QLUULaIsbrj2shifSVos9I6gcn1i0niNHf0SlXZehAbj cFJLaA3DGxlhxb!0z5kO02WgmO7QMu891ZvAsg5j8M3sSparOkBjeV88SHglYV7vJyop BVna2MBGr7MLZO7XvlNUG9ttjz6O34smfjCPMNC35vtTt8BM5fMgrWEUkGNRIgKfECTomGeFfsIo5Zoejtmkjaj3luAQcLTYMAFTZhLHMipI6CiE0Erli1T!t7YKX!hmbytUoOzUEzQEOFRPxn1XEm BnLTAF0DcvdT6TTQ!ur8cGUHU7NbbD9zEbxt3EAHtyQBgimQ6 fiCRR2OI3A7X8QNfz6AhkJh18sp8LfhKQlwpbTp!L0C7BJq6ZSLnnE0tEUhuqQB8FT6Ja wYAWRWtd0Z1QTkiov0Z6hLk0q79K");
        let mut enc23 = cstr!("");
        test_string_cmptlz_enc_cmptlz_dec(enc0);
        test_string_cmptlz_enc_cmptlz_dec(enc1);
        test_string_cmptlz_enc_cmptlz_dec(enc2);
        test_string_cmptlz_enc_cmptlz_dec(enc3);
        test_string_cmptlz_enc_cmptlz_dec(enc4);
        test_string_cmptlz_enc_cmptlz_dec(enc5);
        test_string_cmptlz_enc_cmptlz_dec(enc6);
        test_string_cmptlz_enc_cmptlz_dec(enc7);
        test_string_cmptlz_enc_cmptlz_dec(enc8);
        test_string_cmptlz_enc_cmptlz_dec(enc9);
        test_string_cmptlz_enc_cmptlz_dec(enc10);
        test_string_cmptlz_enc_cmptlz_dec(enc11);
        test_string_cmptlz_enc_cmptlz_dec(enc12);
        test_string_cmptlz_enc_cmptlz_dec(enc13);
        test_string_cmptlz_enc_cmptlz_dec(enc14);
        test_string_cmptlz_enc_cmptlz_dec(enc15);
        test_string_cmptlz_enc_cmptlz_dec(enc16);
        test_string_cmptlz_enc_cmptlz_dec(enc17);
        test_string_cmptlz_enc_cmptlz_dec(enc18);
        test_string_cmptlz_enc_cmptlz_dec(enc19);
        test_string_cmptlz_enc_cmptlz_dec(enc20);
        test_string_cmptlz_enc_cmptlz_dec(enc21);
        test_string_cmptlz_enc_cmptlz_dec(enc22);
        test_string_cmptlz_enc_cmptlz_dec(enc23);
        test_no_memory_leak!();
    }}

    fn test_dir_cmptlz_enc_cmptlz_dec(dirPath: Ptr<u8>) {
        let mut d_name: Ptr<u8>;
        let paths = std::fs::read_dir(dirPath.to_string()).unwrap();
        let mut fullPath: Array<u8, 256> = Array::default();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                continue;
            }
            let d_name = path.file_name().unwrap().to_str().unwrap();
            assert!(c_sprintf_s!(fullPath, 256, cstr!("{}/{}"), dirPath, d_name) > 0);
            test_file_cmptlz_enc_cmptlz_dec(fullPath.cast());
            println!("{} : PASS", d_name);
        }
    }

    #[test]
    fn UT_CMPTLZ_ENCODE_AND_DECODE_FUNC_002() {
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/corpus/Calgary"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/corpus/Canterbury"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/corpus/Misc"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/corpus/Artificial"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/nengyuan"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/shutong"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/wireless"));
        test_dir_cmptlz_enc_cmptlz_dec(cstr!("data/test_file/wuxian"));
        test_no_memory_leak!();
    }
}
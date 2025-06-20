pub mod rapidlz_c;

use rapidlz_c::*;

use crate::translation_utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn UT_Rapidlz_Compress_API_002() {
        assert_eq!(2122219150, RapidlzCompressBound(RAPIDLZ_MAX_INPUT_SIZE!()));
        assert_eq!(0, RapidlzCompressBound(RAPIDLZ_MAX_INPUT_SIZE!() + 1));
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Compress_API_003() {
        let mut src: Ptr<u8> = cstr!("wieruoweiuro12lsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        let i: u32 = RAPIDLZ_READ32BIT!(src + 54);

        cSize = RapidlzCompress(src, dst, srcSize, dstSize, 0);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompress(src, dst, srcSize, dstSize, 11);
        assert_eq!(cSize, 0);

        c_for!(let mut compress_level = 1; compress_level <= 10; compress_level += 1; {
            cSize = RapidlzCompress(NULL!(), NULL!(), 0, 0, compress_level);
            assert_eq!(cSize, 0);

            cSize = RapidlzCompress(NULL!(), dst, srcSize, dstSize, compress_level);
            assert_eq!(cSize, 0);

            cSize = RapidlzCompress(src, NULL!(), srcSize, dstSize, compress_level);
            assert_eq!(cSize, 0);

            cSize = RapidlzCompress(src, dst, 0, dstSize, compress_level);
            assert_eq!(cSize, 0);

            cSize = RapidlzCompress(src, dst, srcSize, 0, compress_level);
            assert_eq!(cSize, 0);

            cSize = RapidlzCompress(src, dst, srcSize, dstSize - 1, compress_level);
            assert_ne!(cSize, 0);
            break;
        });
        c_free!(dst);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Compress_API_004() {
        let mut src: Ptr<u8> =
            cstr!("wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        let mut compareSrc: VoidPtr = NULL!();
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        c_for!(let mut compress_level = 1; compress_level <= 10; compress_level += 1; {
            cSize = RapidlzCompress(src, dst, srcSize, dstSize, compress_level);
            assert_ne!(0, cSize);

            compareSrc = c_malloc!(srcSize);
            assert_ne!(compareSrc, NULL!());

            assert_eq!(srcSize, RapidlzDecompress(dst, compareSrc, cSize, srcSize));
            assert_eq!(0, c_memcmp!(src, compareSrc, srcSize));
            c_free!(compareSrc);
            compareSrc = NULL!();
            break;
        });
        c_free!(dst);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Compress_API_005() {
        let mut src: Ptr<u8> = cstr!("wieruoweiuro12lsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        cSize = RapidlzCompressDefault(NULL!(), NULL!(), 0, 0);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompressDefault(NULL!(), dst, srcSize, dstSize);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompressDefault(src, NULL!(), srcSize, dstSize);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompressDefault(src, dst, 0, dstSize);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompressDefault(src, dst, srcSize, 0);
        assert_eq!(cSize, 0);

        cSize = RapidlzCompressDefault(src, dst, srcSize, dstSize - 1);
        assert_ne!(cSize, 0);
        c_free!(dst);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Compress_API_006() {
        let mut src: Ptr<u8> =
            cstr!("wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        let mut compareSrc: VoidPtr = NULL!();
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        cSize = RapidlzCompressDefault(src, dst, srcSize, dstSize);
        assert_ne!(0, cSize);

        compareSrc = c_malloc!(srcSize);
        assert_ne!(compareSrc, NULL!());

        assert_eq!(srcSize, RapidlzDecompress(dst, compareSrc, cSize, srcSize));
        assert_eq!(0, c_memcmp!(src, compareSrc, srcSize));
        c_free!(compareSrc);
        c_free!(dst);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Compress_API_007() {
        assert_ne!(RapidlzVersionGet(), NULL!());
    }

    fn TestLogFunc(message: Ptr<u8>, size: usize) {
        println!("LogError : {} Length : {}", message, size);
    }

    #[test]
    fn UT_Rapidlz_Compress_API_008() {
        RapidlzLogRegister(func!(TestLogFunc));
        RAPIDLZ_LOG!(0, cstr!("{}"), cstr!("error"));
        RapidlzLogRegister(NULL!());
    }

    #[test]
    fn UT_Rapidlz_Compress_API_009() {
        let mut val: u64 = 0x100000000;
        assert_eq!(32, RapidlzHighBit64(val));

        val = 0xfffffffff;
        assert_eq!(35, RapidlzHighBit64(val));

        val = 0x1000000000000;
        assert_eq!(48, RapidlzHighBit64(val));

        val = 0x8000000000000000;
        assert_eq!(63, RapidlzHighBit64(val));
    }

    #[test]
    fn UT_Rapidlz_Compress_API_010() {
        let mut val: u64 = 0x100000000;
        assert_eq!(32, RapidlzCountTailZero64(val));

        val = 0xffffff000;
        assert_eq!(12, RapidlzCountTailZero64(val));

        val = 0x1000000000001;
        assert_eq!(0, RapidlzCountTailZero64(val));

        val = 0x8000000000000000;
        assert_eq!(63, RapidlzCountTailZero64(val));
    }

    #[test]
    fn UT_Rapidlz_Decompress_API_001() {
        let mut src: Ptr<u8> =
            cstr!("wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        let mut compareSrc: VoidPtr = NULL!();
        let mut retVal: usize;
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        cSize = RapidlzCompressDefault(src, dst, srcSize, dstSize);
        assert_ne!(0, cSize);

        compareSrc = c_malloc!(srcSize);
        assert_ne!(compareSrc, NULL!());

        retVal = RapidlzDecompress(NULL!(), dst, srcSize, cSize);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, NULL!(), srcSize, cSize);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, dst, 0, cSize);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, dst, srcSize, 0);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(NULL!(), NULL!(), 0, 0);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, dst, srcSize - 2, cSize);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, dst, srcSize, cSize - 2);
        assert_eq!(retVal, 0);

        retVal = RapidlzDecompress(compareSrc, dst, srcSize + 5, cSize);
        assert_eq!(retVal, 0);

        c_free!(dst);
        c_free!(compareSrc);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Rapidlz_Decompress_API_002() {
        let mut src: Ptr<u8> =
            cstr!("wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs");
        let mut cSize = 0;
        let mut srcSize = c_strlen!(src);
        let mut dstSize = RapidlzCompressBound(srcSize);
        let mut compareSrc: VoidPtr = NULL!();
        assert!(dstSize < 1024 * 10);
        let mut dst: VoidPtr = c_malloc!(dstSize);
        assert_ne!(dst, NULL!());

        cSize = RapidlzCompressDefault(src, dst, srcSize, dstSize);
        assert_ne!(0, cSize);

        compareSrc = c_malloc!(srcSize);
        assert_ne!(compareSrc, NULL!());

        assert_eq!(srcSize, RapidlzDecompress(dst, compareSrc, cSize, srcSize));
        assert_eq!(0, c_memcmp!(src, compareSrc, srcSize));
        c_free!(compareSrc);
        c_free!(dst);
        test_no_memory_leak!();
    }

    fn CompressAndDecompressFile(iFile: Ptr<u8>) {
        let mut fp: FilePtr;
        let mut src: VoidPtr;
        let mut dst: VoidPtr;
        let mut decompressBuff: VoidPtr;
        let mut readCount: usize;
        let mut srcSize: usize;
        let mut dstCapacity: usize;
        let mut cSize = 0;
        let mut decSize = 0;

        fp = c_fopen!(iFile, cstr!("rb"));
        assert!(fp != NULL!());

        c_fseek!(fp, 0, SEEK_END!());
        srcSize = c_ftell!(fp) as usize;
        c_fseek!(fp, 0, SEEK_SET!());

        src = c_malloc!(srcSize + 1);
        assert_ne!(src, NULL!());

        readCount = c_fread!(src, 1, srcSize, fp);
        assert_eq!(readCount, srcSize);

        dstCapacity = RapidlzCompressBound(srcSize);

        dst = c_malloc!(dstCapacity);
        assert_ne!(dst, NULL!());

        decompressBuff = c_malloc!(srcSize);
        assert_ne!(decompressBuff, NULL!());

        cSize = RapidlzCompress(src, dst, srcSize, dstCapacity, 1);
        assert_ne!(cSize, 0);

        decSize = RapidlzDecompress(dst, decompressBuff, cSize, srcSize);
        assert_eq!(decSize, srcSize);

        if c_memcmp!(src, decompressBuff, srcSize) != 0 {
            c_fclose!(fp);
            c_free!(src);
            c_free!(dst);
            c_free!(decompressBuff);
            println!("error");
            assert!(false);
        }
        c_fclose!(fp);
        c_free!(src);
        c_free!(dst);
        c_free!(decompressBuff);
    }

    fn CompressAndDecompressDir(dirPath: Ptr<u8>) {
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
            CompressAndDecompressFile(fullPath.cast());
            println!("{} : PASS", d_name);
        }
    }

    #[test]
    fn UT_Rapidlz_Decompress_API_003() {
        RapidlzLogRegister(func!(TestLogFunc));
        CompressAndDecompressDir(cstr!("data/test_file/corpus/Calgary"));
        CompressAndDecompressDir(cstr!("data/test_file/corpus/Canterbury"));
        CompressAndDecompressDir(cstr!("data/test_file/corpus/Misc"));
        CompressAndDecompressDir(cstr!("data/test_file/corpus/Artificial"));
        CompressAndDecompressDir(cstr!("data/test_file/nengyuan"));
        CompressAndDecompressDir(cstr!("data/test_file/shutong"));
        CompressAndDecompressDir(cstr!("data/test_file/wireless"));
        CompressAndDecompressDir(cstr!("data/test_file/wuxian"));
        test_no_memory_leak!();
    }
}

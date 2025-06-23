pub mod bzp_c;

use bzp_c::*;

use crate::translation_utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn UT_BwtInit_FUNC_001() {
        let mi_level = BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!();
        let mx_level = BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!();
        c_for!(let mut i = mi_level; i <= mx_level; i += 1; {
            let bwt = BzpBlockSortInit(i);
            assert!(bwt != NULL!());
            BzpBwtFinish(bwt);
        });
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BwtMain_FUNC_001() {
        let level = 1;
        let mut bwt = BzpBlockSortInit(level);
        let mut ss: Ptr<u8> = cstr!("vxevfdoqrscqyumzltnjdozcjzhexqvdqvlpkxauluhqwvzodicdcexmlloskrqswogiwdgnymgjznnmqdvafxjzwebjfpqsgfci");
        let mut res: Array<i32, 100> = arr![
            83, 38, 90, 50, 52, 98, 23, 10, 51, 69, 48, 5,  20, 31, 81, 89, 2,  53, 27, 97, 4,  92, 84, 96, 66,
            74, 70, 26, 42, 49, 99, 67, 19, 91, 24, 75, 86, 60, 36, 56, 57, 34, 16, 40, 73, 55, 79, 14, 18, 78,
            77, 71, 47, 65, 6,  58, 21, 35, 93, 80, 7,  94, 62, 29, 32, 43, 11, 61, 8,  9,  95, 59, 63, 17, 41,
            39, 13, 82, 30, 3,  33, 0,  45, 68, 88, 64, 44, 37, 1,  85, 54, 28, 72, 12, 22, 25, 15, 76, 46, 87,
        ];
        bwt.nBlock = 0;
        c_for!(let mut i = 0; ss[i] != 0; i += 1; {
            bwt.nBlock += 1;
            bwt.block[i] = ss[i];
        });
        BzpBlockSortMain(bwt);
        c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
            assert_eq!(bwt.sortBlock[i], res[i]);
        });
        BzpBwtFinish(bwt);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BwtQsort_FUNC_001() {
        let mut ss: Ptr<u8> = cstr!("qwertyuioaspdgcfvxbhzjnkml");
        let mut sortBlock: Array<i32, 30> = arr![0; 30];
        let mut idx: Array<i32, 30> = arr![0; 30];
        let mut n = c_strlen!(ss);
        c_for!(let mut i = 0; i < n; i += 1; {
            sortBlock[i] = i as i32;
            idx[i] = ss[i] as i32 - 'a' as i32;
        });
        BzpQuickSort(sortBlock.cast(), idx.cast(), 0, n as i32 - 1);
        c_for!(let mut i = 0; i < n; i += 1; {
            assert_eq!(ss[sortBlock[i as usize]], i as u8 + 'a' as u8);
        });
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BwtQsort_FUNC_002() {
        let mut ss: Ptr<u8> = cstr!("qwedqsewdfubasqsbwvb");
        let mut sortBlock: Array<i32, 30> = arr![0; 30];
        let mut idx: Array<i32, 30> = arr![0; 30];
        let mut n = c_strlen!(ss);
        let mut res: Array<i32, 20> = arr![12, 16, 19, 11, 3, 8, 2, 6, 9, 0, 14, 4, 5, 13, 15, 10, 18, 1, 17, 7];
        c_for!(let mut i = 0; i < n; i += 1; {
            sortBlock[i] = i as i32;
            idx[i] = ss[i] as i32 - 'a' as i32;
        });
        BzpQuickSort(sortBlock.cast(), idx.cast(), 0, n as i32 - 1);
        c_for!(let mut i = 0; i < n; i += 1; {
            assert_eq!(sortBlock[i as usize], res[i as usize]);
        });
        test_no_memory_leak!();
    }
    
    #[test]
    fn UT_BzpBwtDecodeInit_FUNC_001() {
        let mi_level = BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!();
        let mx_level = BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!();
        c_for!(let mut i = mi_level; i <= mx_level; i += 1; {
            let bwt = BzpBwtDecodeInit(i);
            assert!(bwt != NULL!());
            BzpBwtDecodeFinish(bwt);
        });
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BzpBwtDecodeMain_FUNC_001() {
        let mut ss: Ptr<u8> = cstr!("vxeidfzscwofjvqwxchgvjasomdzudcgnbcgxspmlvzuyxnutnzgzwdldlfmoprxdhckqrqoqllaydqeqiwizsqkvfeenqojmjvj");
        let mut res: Ptr<u8> = cstr!("vxevfdoqrscqyumzltnjdozcjzhexqvdqvlpkxauluhqwvzodicdcexmlloskrqswogiwdgnymgjznnmqdvafxjzwebjfpqsgfci");
        let mut debwt = BzpBwtDecodeInit(1);
        assert!(debwt != NULL!());
        debwt.oriPtr = 81;
        c_for!(let mut i = 0; ss[i] != 0; i += 1; {
            let idx = debwt.nBlock as usize;
            debwt.block[idx] = ss[i];
            debwt.nBlock += 1;
        });
        BzpBwtDecode(debwt);
        c_for!(let mut i = 0; ss[i] != 0; i += 1; {
            assert_eq!(debwt.deCode[i], res[i]);
        });
        BzpBwtDecodeFinish(debwt);
        test_no_memory_leak!();
    }
    
    #[test]
    fn UT_BzpHuffmanInit_FUNC_001() {
        let mut huffman = BzpHuffmanGroupsInit(9);
        assert!(huffman != NULL!());
        BzpHuffmanGroupsFinish(huffman);

        let mut huffman = BzpHuffmanGroupsInit(3);
        assert!(huffman != NULL!());
        BzpHuffmanGroupsFinish(huffman);

        let mut huffman = BzpHuffmanGroupsInit(5);
        assert!(huffman != NULL!());
        BzpHuffmanGroupsReset(huffman, 7);
        assert_eq!(huffman.alphaSize, 7);
        assert_eq!(huffman.huffmanGroups[0].alphaSize, 7);
        BzpHuffmanGroupsReset(huffman, 300);
        assert_eq!(huffman.alphaSize, 7);
        BzpHuffmanGroupsFinish(huffman);

        let mut huffman = BzpHuffmanGroupsInit(10);
        assert!(huffman == NULL!());
        let mut huffman = BzpHuffmanGroupsInit(0);
        assert!(huffman == NULL!());
        test_no_memory_leak!();
    }
    
    #[test]
    fn UT_BzpHuffmanDecodeInit_FUNC_001() {
        let mut huffman = BzpHuffmanDecodeInit(9);
        assert!(huffman != NULL!());
        BzpHuffmanDecodeFinish(huffman);
        let mut huffman = BzpHuffmanDecodeInit(1);
        assert!(huffman != NULL!());
        BzpHuffmanDecodeFinish(huffman);
        let mut huffman = BzpHuffmanDecodeInit(0);
        assert!(huffman == NULL!());
        let mut huffman = BzpHuffmanDecodeInit(10);
        assert!(huffman == NULL!());
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BzpGetHuffmanGroups_FUNC_001() {
        let huffman = BzpGetHuffmanGroups(0);
        assert_eq!(huffman, 2);
        BzpGetHuffmanGroups(1);
        assert_eq!(huffman, 2);
        let huffman = BzpGetHuffmanGroups(100);
        assert_eq!(huffman, 2);
        let huffman = BzpGetHuffmanGroups(200);
        assert_eq!(huffman, 3);
        let huffman = BzpGetHuffmanGroups(400);
        assert_eq!(huffman, 3);
        let huffman = BzpGetHuffmanGroups(600);
        assert_eq!(huffman, 4);
        let huffman = BzpGetHuffmanGroups(800);
        assert_eq!(huffman, 4);
        let huffman = BzpGetHuffmanGroups(1200);
        assert_eq!(huffman, 5);
        let huffman = BzpGetHuffmanGroups(1800);
        assert_eq!(huffman, 5);
        let huffman = BzpGetHuffmanGroups(2400);
        assert_eq!(huffman, 6);
        let huffman = BzpGetHuffmanGroups(3000);
        assert_eq!(huffman, 6);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BzpMtfInit_FUNC_001() {
        let mi_level = BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!();
        let mx_level = BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!();
        c_for!(let mut i = mi_level; i <= mx_level; i += 1; {
            let mtf = BzpMtfInit(i);
            assert!(mtf != NULL!());
            BzpMtfFinish(mtf);
        });
        test_no_memory_leak!();
    }

    #[test]
    fn UT_BzpMtfMain_FUNC_001() {
        let mut mtf = BzpMtfInit(1);
        assert!(mtf != NULL!());
        let mut res: Array<i32, 6> = arr![3, 0, 3, 3, 1, 4];
        mtf.block = cstr!("banana");
        let mut map = arr![5, 3, 1, 0, 4, 2];
        mtf.map = map.cast();
        let mut inuse = arr![false; 256];
        mtf.inUse = inuse.cast();
        mtf.nBlock = 0;
        c_for!(let mut i = 0; c_strlen!(mtf.block) > i; i += 1; {
            mtf.nBlock += 1;
            let index = mtf.block[i];
            mtf.inUse[index] = true;
        });
        BzpMtfMain(mtf);
        c_for!(let mut i = 0; i < mtf.nMtf; i += 1; {
            assert_eq!(mtf.mtfV[i], res[i]);
        });
        BzpMtfFinish(mtf);
        test_no_memory_leak!();
    }

    #[test]
    fn UT_MtfBzpNumEncode_FUNC_001() {
        let mut res: Array<Ptr<u8>, 8> = arr![
            cstr!(""), cstr!("0"), cstr!("1"), cstr!("00"), cstr!("10"), cstr!("01"), cstr!("11"), cstr!("000")
        ];
        let mut mtf = BzpMtfInit(1);
        assert!(mtf != NULL!());
        let mut mtfV: Array<i32, 100> = arr![0; 100];
        c_free!(mtf.mtfV);
        mtf.mtfV = mtfV.cast();
        c_for!(let mut num = 1; num <= 7; num += 1; {
            BzpNumEncode(mtf, num);
            assert_eq!(mtf.nMtf, c_strlen!(res[num]) as i32);
            c_for!(let mut i = 0; i < mtf.nMtf; i += 1; {
                assert_eq!(mtf.mtfV[i], (res[num][i] - '0' as u8) as i32);
            });
            mtf.nMtf = 0;
            c_memset!(mtf.mtfFreq.cast::<Ptr<i32>>(), 0, c_sizeofval!(mtf.mtfFreq));
            c_memset!(mtf.mtfV.cast::<Ptr<i32>>(), 0, c_sizeofval!(mtf.mtfV));
        });
        mtf.mtfV = NULL!();
        BzpMtfFinish(mtf);
        test_no_memory_leak!();
    }

    fn CompressAndDecompressFile(mut iFile: Ptr<u8>, mut oFile: Ptr<u8>, mut tFile: Ptr<u8>, mut blockSize: i32) {
        let mut fp: FilePtr;
        let mut fp1: FilePtr;
        let mut src: Ptr<u8>;
        let mut dst: Ptr<u8>;
        let mut srcSize: usize;
        let mut dstSize: usize;
        let mut readCount: usize;
        fp = c_fopen!(iFile, cstr!("rb"));
        assert!(fp != NULL!());

        c_fseek!(fp, 0, SEEK_END!());
        srcSize = c_ftell!(fp) as usize;
        c_fseek!(fp, 0, SEEK_SET!());
        src = c_malloc!(srcSize + 1);
        assert_ne!(src, NULL!());

        readCount = c_fread!(src, 1, srcSize, fp);
        assert_eq!(readCount, srcSize);
        c_fclose!(fp);

        let ret = BzpCompressStream(iFile, oFile, blockSize);
        assert_eq!(ret, BZP_OK!());
        let ret1 = BzpDeCompressStream(oFile, tFile);
        assert_eq!(ret1, BZP_OK!());

        fp1 = c_fopen!(tFile, cstr!("rb"));
        assert!(fp1 != NULL!());

        c_fseek!(fp1, 0, SEEK_END!());
        dstSize = c_ftell!(fp1) as usize;
        c_fseek!(fp1, 0, SEEK_SET!());

        dst = c_malloc!(dstSize + 1);
        assert_ne!(dst, NULL!());

        readCount = c_fread!(dst, 1, dstSize, fp1);
        assert_eq!(readCount, dstSize);
        c_fclose!(fp1);

        assert_eq!(srcSize, dstSize);
        assert_eq!(c_memcmp!(src, dst, srcSize), 0);
        c_free!(src); 
        c_free!(dst);
        c_remove!(oFile);
        c_remove!(tFile);
    }

    fn CompressAndDecompressDir(dirPath: Ptr<u8>) {
        let mut d_name: Ptr<u8>;
        let paths = std::fs::read_dir(dirPath.to_string()).unwrap();
        let mut fullPath: Array<u8, 256> = Array::default();
        let mut cnt = 0;
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                continue;
            }
            cnt = cnt % 9 + 1;
            let d_name = path.file_name().unwrap().to_str().unwrap();
            assert!(c_sprintf_s!(fullPath, 256, cstr!("{}/{}"), dirPath, d_name) > 0);
            CompressAndDecompressFile(fullPath.cast(), cstr!("data/test_file_output/tmp.bz2"), cstr!("data/test_file_output/tmp"), cnt);
            println!("{} : PASS", d_name);
        }
    }

    #[test]
    fn UT_Stream_PARAM_ERROR_FUNC() {
        let inName: Ptr<u8> = cstr!("data/test_file/corpus/Misc/pi.txt");
        let outName: Ptr<u8> = cstr!("data/output/pi.txt.bz2");
        let blockSize = 9;
        let mut ret = BZP_OK!();

        ret = BzpCompressStream(NULL!(), outName, blockSize);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        ret = BzpCompressStream(inName, NULL!(), blockSize);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        ret = BzpCompressStream(inName, outName, 10);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        ret = BzpCompressStream(inName, outName, 0);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        ret = BzpCompressStream(inName, outName, -5);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        test_no_memory_leak!();
    }
    
    #[test]
    fn UT_Stream_IO_ERROR_FUNC() {
        let errorinname: Ptr<u8> = cstr!("data/test_file/corpus/Misc/pi2.txt");
        let outName: Ptr<u8> = cstr!("data/output/pi.txt.bz2");
        let blockSize = 9;
        let mut ret = BZP_OK!();
        ret = BzpCompressStream(errorinname, outName, blockSize);
        assert_eq!(ret, BZP_ERROR_IO!());
        test_no_memory_leak!();
    }

    #[test]
    fn UT_DeComStream_PARAM_ERROR_FUNC() {
        let inName: Ptr<u8> = cstr!("data/test_file/compressed/Normal/bible.txt.bz2");
        let outName: Ptr<u8> = cstr!("data/output/bible_param_error.txt");
        let mut ret = BZP_OK!();
        ret = BzpDeCompressStream(NULL!(), outName);
        assert_eq!(ret, BZP_ERROR_PARAM!());
        ret = BzpDeCompressStream(inName, NULL!());
        assert_eq!(ret, BZP_ERROR_PARAM!());
        test_no_memory_leak!();
    }

    #[test]
    fn UT_DeComStream_IO_ERROR_FUNC() {
        let errorinname: Ptr<u8> = cstr!("data/test_file/compressed/Normal/bible2.txt.bz2");
        let outName: Ptr<u8> = cstr!("data/output/bible_io_error.txt");
        let mut ret = BZP_OK!();
        ret = BzpDeCompressStream(errorinname, outName);
        assert_eq!(ret, BZP_ERROR_IO!());
        test_no_memory_leak!();
    }

    #[test]
    fn UT_Stream_Run_Ok_FUNC() {
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
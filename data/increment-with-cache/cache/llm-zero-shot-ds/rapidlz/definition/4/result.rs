type RapidlzDecompressFunc = fn(src: *const i8, dest: *mut i8, srcSize: i32, outBufferSize: i32, dictStart: *const i8, dictSize: i32) -> i32;

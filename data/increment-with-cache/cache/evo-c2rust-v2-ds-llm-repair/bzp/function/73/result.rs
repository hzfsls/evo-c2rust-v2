pub fn BzpWriteInt32(mut val: i32, mut data: Ptr<BzpOutComdata>) {
    BzpWriteToArray(((val >> BZP_BITS24!()) & 0xff).cast::<i32>(), BZP_BITS8!(), data);
    BzpWriteToArray(((val >> BZP_BITS16!()) & 0xff).cast::<i32>(), BZP_BITS8!(), data);
    BzpWriteToArray(((val >> BZP_BITS8!()) & 0xff).cast::<i32>(), BZP_BITS8!(), data);
    BzpWriteToArray((val & 0xff).cast::<i32>(), BZP_BITS8!(), data);
}

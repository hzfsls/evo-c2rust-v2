pub fn RapidlzIsLE() -> i32 {
    #[cfg(any(__GNUC__, __clang__))]
    {
        return (__BYTE_ORDER__!() == __ORDER_LITTLE_ENDIAN__!()).cast();
    }
    #[cfg(not(any(__GNUC__, __clang__)))]
    {
        let mut n: i32 = 1;
        return (*c_ref!(n).cast::<Ptr<u8>>()).cast::<i32>();
    }
}

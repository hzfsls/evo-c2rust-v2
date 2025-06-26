pub fn arraylist_index_of(mut arraylist: Ptr<ArrayList>, mut callback: ArrayListEqualFunc, mut data: ArrayListValue) -> i32 {
    let mut i: u32 = Default::default();

    c_for!(i = 0; i < arraylist.length; i.prefix_plus_plus(); {
        if (callback(arraylist.data[i], data) != 0).as_bool() {
            return i.cast::<i32>();
        }
    });

    return -1;
}

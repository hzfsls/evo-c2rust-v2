pub fn sortedarray_insert(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue) -> i32 {
    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length.cast();
    let mut index: u32 = 0;
    right = if right > 1 { right } else { 0 };
    while (left != right).as_bool() {
        index = (left + right) / 2;
        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if order < 0 {
            right = index.cast();
        } else if order > 0 {
            left = (index + 1).cast();
        } else {
            break;
        }
    }
    if (sortedarray.length > 0).as_bool() && ((sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()) > 0).as_bool() {
        index += 1;
    }
    if (sortedarray.length + 1 > sortedarray._alloced).as_bool() {
        let mut newsize: u32;
        let mut data: Ptr<SortedArrayValue>;
        newsize = sortedarray._alloced * 2;
        data = c_realloc!(sortedarray.data, c_sizeof!(SortedArrayValue) * newsize);
        if (data == NULL!()).as_bool() {
            return 0;
        } else {
            sortedarray.data = data.cast();
            sortedarray._alloced = newsize.cast();
        }
    }
    c_memmove!(
        c_ref!(sortedarray.data[index + 1]).cast(),
        c_ref!(sortedarray.data[index]).cast(),
        (sortedarray.length - index) * c_sizeof!(SortedArrayValue)
    );
    sortedarray.data[index] = data.cast();
    sortedarray.length += 1;
    return 1;
}

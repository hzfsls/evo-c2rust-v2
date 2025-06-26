pub fn sortedarray_remove_range(mut sortedarray: Ptr<SortedArray>, mut index: u32, mut length: u32) {
    if (index > sortedarray.length || index + length > sortedarray.length) {
        return;
    }
    c_memmove_s!(
        c_ref!(sortedarray.data[index]),
        (sortedarray.length - index) * c_sizeof!(SortedArrayValue),
        c_ref!(sortedarray.data[index + length]),
        (sortedarray.length - (index + length)) * c_sizeof!(SortedArrayValue)
    );
    sortedarray.length -= length;
}

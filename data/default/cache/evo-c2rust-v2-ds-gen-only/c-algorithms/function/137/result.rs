pub fn arraylist_remove_range(mut arraylist: Ptr<ArrayList>, mut index: u32, mut length: u32) {
    if (index > arraylist.length).as_bool() || (index + length > arraylist.length).as_bool() {
        return;
    }
    c_memmove!(
        c_ref!(arraylist.data[index]).cast(),
        c_ref!(arraylist.data[index + length]).cast(),
        (arraylist.length - (index + length)) * c_sizeof!(ArrayListValue)
    );
    arraylist.length -= length;
}

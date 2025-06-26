pub fn slist_prepend(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut newentry: Ptr<SListEntry>;

    newentry = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()).as_bool() {
        return NULL!();
    }

    newentry.data = data.cast();

    newentry.next = *list;
    *list = newentry.cast();

    return newentry.cast();
}

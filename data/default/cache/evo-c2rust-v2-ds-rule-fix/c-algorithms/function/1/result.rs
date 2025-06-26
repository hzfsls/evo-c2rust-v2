pub fn list_prepend(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut newentry: Ptr<ListEntry>;

    if (list == NULL!()) {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;

    if (*list != NULL!()) {
        (*list).prev = newentry;
    }
    newentry.prev = NULL!();
    newentry.next = *list;
    *list = newentry;

    return newentry;
}

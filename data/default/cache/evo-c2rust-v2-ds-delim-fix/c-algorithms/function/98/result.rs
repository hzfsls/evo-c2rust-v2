pub fn int_hash(mut vlocation: Ptr<Void>) -> u32 {
    let mut location: Ptr<i32>;
    location = vlocation.cast::<Ptr<i32>>();
    return (*location).cast::<u32>();
}

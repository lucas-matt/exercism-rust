pub fn find<R: AsRef<[T]>, T:PartialOrd>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None
    }
    let idx = array.len() / 2;
    match &array[idx] {
        n if &key < n => find(&array[0..idx], key),
        n if &key > n => find(&array[idx+1..], key).map(|i| i + idx + 1),
        _ => Some(idx)
    }
}

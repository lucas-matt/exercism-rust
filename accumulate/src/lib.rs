/// What should the type of _function be?
pub fn map<T, S>(input: Vec<T>, mut function: impl FnMut (T) -> S) -> Vec<S> {
    let mut result:Vec<S> = Vec::new();
    for entry in input {
        result.push(function(entry));
    }
    result
}

pub mod parser;

pub fn clone_slice_into_index<T: Clone>(source: &[T], destination: &mut [T], index: usize) {
    let mut current = index;
    while current < source.len() && index + current < destination.len() {
        destination[index + current] = source[current].clone();
        current += 1;
    }
}

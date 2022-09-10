fn lomuto_partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let pivot = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if slice[j] < slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot);

    i
}

pub fn quick_sort<T: Ord + Clone>(slice: &mut [T]) {
    if slice.len() > 1 {
        let pi = lomuto_partition(slice);
        quick_sort(&mut slice[..pi]);
        quick_sort(&mut slice[pi+1..]);
    }
}
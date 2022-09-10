fn hoare_partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let pivot = 0;
    let (mut left, mut right) = (pivot, slice.len() - 1);
    
    loop {
        while slice[left] < slice[pivot] {
            left += 1
        }
        
        while slice[right] > slice[pivot] {
            right -= 1
        }
        
        if left >= right {
            return right;
        }

        slice.swap(left, right);    
    }
}

pub fn quick_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() > 1 {
        let pi = hoare_partition(slice);
        quick_sort(&mut slice[..pi]);
        quick_sort(&mut slice[pi+1..]);
    }
}
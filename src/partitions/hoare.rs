fn hoare_partition<T: Ord + Clone>(slice: &mut [T], left: usize, right: usize) -> usize {
    let pivot = slice[left].clone();
    let (mut i, mut j) = (left, right);
    
    loop {
        while slice[i] < pivot {
            i += 1
        }
        
        while slice[j] > pivot {
            j -= 1
        }
        
        if i >= j {
            return j;
        }

        slice.swap(i, j);
    }
}

fn _quick_sort<T: Ord + Clone>(slice: &mut [T], left: usize, right: usize) {
        if left < right {
            let pi = hoare_partition(slice, left, right);
            _quick_sort(slice, left, pi);
            _quick_sort(slice, pi+1, right);
        }
}

pub fn quick_sort<T: Ord + Clone>(slice: &mut [T]) {
    let (left, right) = (0_usize, slice.len() - 1);
    _quick_sort(slice, left, right)
}
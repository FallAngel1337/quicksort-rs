// TODO: Add a proc-macro for #[derive(QuickSort)]

mod partitions;

pub trait QuickSort {
    fn quick_sort(self);
}

impl<T: Ord> QuickSort for &[T] {
    #[cfg(feature = "hoare")]
    fn quick_sort(self) {
        partitions::hoare::quick_sort(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickSort;

    #[test]
    fn test() {
        let vec = vec![5, 4, 3, 1, 2];
        vec.quick_sort()
    }
}

mod partitions;

pub trait QuickSort {
    fn quick_sort(self);
}

impl<T: Ord + Clone> QuickSort for &mut [T] {
    #[cfg(feature = "hoare")]
    fn quick_sort(self) {
        partitions::hoare::quick_sort(self)
    }
    #[cfg(feature = "lomuto")]
    fn quick_sort(self) {
        partitions::lomuto::quick_sort(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickSort;
    use rand::Rng;

    #[test]
    fn simple_vec_sort_test() {
        let mut vec = vec![5, 4, 3, 1, 2];
        vec.quick_sort();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn random_vec_sort_test() {
        let mut rng = rand::thread_rng();
        let mut a = (0..100).map(|_| rng.gen::<u32>()).collect::<Vec<_>>();
        let mut b = a.clone();

        a.quick_sort();
        b.sort();

        assert_eq!(a, b);
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::is_sorted;

    use super::bubble_sort;


    #[test]
    fn descending() {
        let mut v1 = vec![6,5,4,3,2,1];
        bubble_sort(&mut v1);
        assert!(is_sorted(&v1));
    }
}
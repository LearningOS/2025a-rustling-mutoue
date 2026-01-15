/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() <= 1 {
        return;
    }
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: PartialOrd>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let divide_index = partition(array, low, high);
        if divide_index > 0 {
            quick_sort(array, low, divide_index - 1);
        }
        quick_sort(array, divide_index + 1, high);
    }
}

fn partition<T: PartialOrd>(array: &mut [T], low: usize, high: usize) -> usize {
    if low >= high {
        return high;
    }

    // pick first value as divider
    let mut i = low + 1;
    let mut j = high;

    while i <= j {
        while i <= j && array[j] >= array[low] && j > 0 {
            j -= 1;
        }
        while i <= j && array[i] <= array[low] {
            i += 1;
        }
        if i < j {
            array.swap(i, j);
        }
    }
    array.swap(low, j);
    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}

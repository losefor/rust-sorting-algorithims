pub fn sort(array: &mut [i32]) {
    // Get length of an array
    let length = array.len();

    // Flag for skipping last redundant iteration
    let mut _is_sorted = false;

    for iteration in 0..length {
        _is_sorted = true;
        for index in 0..length - iteration - 1 {
            if array[index] > array[index + 1] {
                swap(array, index as usize, index + 1);
                _is_sorted = false;
            }
        }
        if _is_sorted {
            return;
        }
    }
}

fn swap(array: &mut [i32], index1: usize, index2: usize) {
    array[index1] += array[index2];
    array[index2] = array[index1] - array[index2];
    array[index1] -= array[index2];
}

// 3 , 4
// first = 3 + 4 = 7
// second = 7 - 4 = 3
// first = 7 - second = 4

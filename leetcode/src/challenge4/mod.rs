pub fn test_challenge(){
    println!("Result: {}", find_median_sorted_arrays(vec![1, 3, 9, 4], vec![10, 8, 2, 5]));


}

fn partition(array: &mut Vec<i32>, low: i32, high: i32) -> i32{
    let pivot = array[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if array[j as usize] <= pivot {
            i += 1;
            let aux = array[j as usize];
            array[j as usize] = array[i as usize];
            array[i as usize] = aux;
        }        
    }

    let aux = array[high as usize];
    array[high as usize] = array[(i+1) as usize];
    array[(i+1) as usize] = aux;

    return i+1;

}


fn quick_sort(array: &mut Vec<i32>, low: i32, high: Option<i32>){
    let mut used_high: i32 = 0;
    if let Some(value) = high {
        used_high = value;
    }
    else {
        used_high = (array.len() - 1) as i32;
    }

    if low < used_high {
        let pivot_index = partition(array, low, used_high);
        quick_sort(array, low, Some(pivot_index-1));
        quick_sort(array, pivot_index+1, high);
    }

}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut combined_vec: Vec<i32> = nums1.clone();
    combined_vec.append(&mut nums2.clone());
    //TODO: write a sorting algorithm and get the median element from the sorted vector
    quick_sort(&mut combined_vec, 0, None);

    // println!("sorted array: {:?}", combined_vec);
    let half_idx = combined_vec.len() / 2;
    if combined_vec.len() % 2 == 0 { //if it has even amount of elements
        
        let middle1 = combined_vec[half_idx];
        let middle2 = combined_vec[half_idx-1];
        let values: f64 = ((middle1+middle2) as f64) / 2.0;
        return values;
    }
    else {
        let middle = combined_vec[half_idx];
        return middle as f64;
    }

}
pub fn test_challenge(){
    println!("Result: {}", find_median_sorted_arrays(vec![1, 2], vec![3, 4]));


}


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut combined_vec: Vec<i32> = nums1.clone();
    combined_vec.append(&mut nums2.clone());
    //TODO: write a sorting algorithm and get the median element from the sorted vector
    

    0.0
}
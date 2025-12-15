pub fn sum_array_1(arr: &[u64]) -> u64 {
    let mut result = 0u64;
    for i in 0..arr.len() {
        result += arr[i];
    }
    result
}

pub fn sum_array_2(arr: &[u64], n: u64) -> u64 {
    // if arr.len() < n {
    //     return 0;
    // }
    let mut result = 0u64;
    for i in 0..n as usize {
        result += arr[i];
    }
    result
}

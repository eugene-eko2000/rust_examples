mod array_test;

fn main() {
    let arr: Vec<u64> = (0..1000000000).collect();

    let start1 = std::time::Instant::now();
    let sum1 = array_test::sum_array_1(&arr);
    let duration1 = start1.elapsed();
    println!("Sum using sum_array_1: {}", sum1);
    println!("Time taken by sum_array_1: {:?}", duration1);
    let start2 = std::time::Instant::now();
    let sum2 = array_test::sum_array_2(&arr, 1000000000);
    let duration2 = start2.elapsed();
    println!("Sum using sum_array_2: {}", sum2);
    println!("Time taken by sum_array_2: {:?}", duration2);
}

pub fn update_elements(arr: &mut [i32]) {
    if arr.len() < 5 { return; }
    arr[0] += 10;
    arr[1] += 10;
    arr[2] += 10;
    arr[3] += 10;
    arr[4] += 10;
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    update_elements(&mut data);
    println!("{:?}", data);
}

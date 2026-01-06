use std::ops::{Add, Mul, Div};

fn math_ops_sugar(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a + b * c / d
}

fn math_ops_no_sugar(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a.add(b).mul(c).div(d)
}

fn main() {
    let res1 = math_ops_sugar(1, 2, 3, 4);
    let res2 = math_ops_no_sugar(1, 2, 3, 4);
    println!("Result1: {}, result2: {}", res1, res2);
}

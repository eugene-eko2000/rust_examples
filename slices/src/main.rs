// use rayon::prelude::*;

// fn with_direct_refs_invalid() {

//     let mut src2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let r1 = &mut src2[0..3];
//     let r2 = &mut src2[3..6];
//     r1[0] *= 10;
//     r1[1] *= 10;
//     r1[2] *= 10;
//     r2[0] *= 10;
//     r2[1] *= 10;
//     r2[2] *= 10;

// }

fn with_direct_refs_unsafe() {


    let mut src2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let p1 = &mut src2[0..7] as *mut [i32];
    let p2 = &mut src2[5..10] as *mut [i32];

    let r1: &mut [i32] = unsafe { &mut *p1 };
    let r2: &mut [i32] = unsafe { &mut *p2 };

    r1[0] *= 10;
    r1[1] *= 10;
    r1[2] *= 10;
    r1[5] *= 10;
    r1[6] *= 10;
    r2[0] *= 10;
    r2[1] *= 10;
    r2[2] *= 10;

    println!("{:?}", src2);

}

fn with_split_at_mut() {
    let mut src3 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = src3.split_at_mut(5);
    left[0] *= 10;
    left[1] *= 10;
    left[2] *= 10;
    right[0] *= 10;
    right[1] *= 10;
    right[2] *= 10;
    println!("{:?}", src3);
}

fn main() {
    with_direct_refs_unsafe();
    with_split_at_mut();
}

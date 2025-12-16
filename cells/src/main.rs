use std::cell::{Cell, RefCell};

fn main() {
    let c = Cell::new(5);
    println!("{}", c.get());
    let v = c.take();
    println!("{}, {:?}", v, c);
    c.set(10);
    let v = c.into_inner();
    println!("{}", v);

    let rc = RefCell::new(vec![1, 2, 3]);
    {
        let mut br1 = rc.borrow_mut();
        br1.push(4);
    }
    {
        let br = rc.borrow();
        println!("{:?}", br);
    }
}

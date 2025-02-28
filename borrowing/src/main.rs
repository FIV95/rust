fn modify(value: &mut i32) {
    *value += 1;
}

fn main() {
    let mut num = 10;

    let ref1 = &mut num;
    let ref1: &mut i32 = &num

    modify(ref1);
    let ref2 = &mut num; // ❌ ERROR: Cannot borrow `num` as mutable more than once
    modify(ref2);

    println!("{}", num);
}

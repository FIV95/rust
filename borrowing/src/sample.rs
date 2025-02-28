fn modify_value(value: &mut i32) {
    *value += 10; // 🚀 Modify the borrowed value
}

fn main() {
    let mut num = 5;

    let ref1 = &num; // Immutable borrow
    println!("Before modification: {}", ref1);

    modify_value(&mut num); // ❌ ERROR: Cannot borrow `num` as mutable because it is also borrowed as immutable

    println!("After modification: {}", num);
}

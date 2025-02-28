fn main() {

    /// Array of 5 elements of type i32
    let x: [i32; 5] = [1, 2, 3, 4, 5];

    /// Array of 500 elements of type i32 initialized to 0
    let y: [i32; 500] = [0; 500];

    /// Vector of i32 using macro vec!
    let z: Vec<i32> = vec![1, 2, 3, 4, 5];

    /// Vector of i32 with no elements
    let mut a: Vec<i32> = Vec::new();

    /// Vector of 10 elements of type i32, all initialized to 0
    let b: Vec<i32> = vec![0; 10];

}

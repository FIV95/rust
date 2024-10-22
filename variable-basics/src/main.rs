fn main() {
    let name = String::from("Frankie");
    let mut age: i32 = 29;
    println!("{} {}", name, age);
    age += 1;
    let new_name = name;
    println!("{}", new_name);
}

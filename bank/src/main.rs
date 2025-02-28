#[derive(Debug)]
pub struct Person {
    name:String,
    age:i32,
    children: i32,
}

#[derive(Debug)]
pub enum Color{
    Red,
    Green,
    Blue,
}

impl Person {
    pub fn print(self)->String{
        return format!("name = {}, age = {}, has {} children", self.name, self.age, self.children);
    }
}

fn main() {
    let p = Person {
        name:"matt".to_string(),
        age:35,
        children:4,
    };
    let c = Color::Red("hello".to_string());
    println!("Hello, people, from {:?}!",p);
    match c {
        Color::Red(s) => println!("It's red"),
        Color::Green => println!("It's green"),
        Color::Blue=> println!("It's blue"),
}

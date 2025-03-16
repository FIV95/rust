fn main() {
   sing(10); 
}

fn sing(start_day: u8) {
    for day in start_day as usize..=12 {
        chorus(day);
        for i in (0..day).rev() {
            println!("{}", if i == 0 {format!("And {}", VERSES[i]) } else {VERSES[i].to_string() });
    }
        println!();
    }
}

fn chorus(day: usize) {
    let result: String = match day {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "nineth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => unreachable!()
    };
    println!(
        "On the {} day of Christmas\nmy true love gave to me", result);
}
static VERSES :[&str; 12] = [
"a partridge in a pear tree.",
"Two Turtle doves,",
"Three French Hens,",
"Four calling birds,",
"Five golden rings,",
"Six geese a-laying,",
"Seven swans a-swimming,",
"Eight maids a-milking,",
"Nine ladies dancing,",
"Ten lords a-leaping,",
"Eleven pipers piping,",
"Twelve drummers drumming,",
];

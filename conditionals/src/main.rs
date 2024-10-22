fn main() {

    let mut age: i32 = 29;
    let year: i32 = 1990;
    let hoursPlayed: i32 = 100;
    if age >= 30
    {
        println!("Age is 30 or more.");
    }
    else
    {
        println!("Age is less than 30.");
    }

    let choice = String::from("");

    // show the difference of a traditional switch statement

    match year{
        1995 => println!("1995"),
        1996 => println!("1996"),
        1997=> println!("1997"),
        2000 | 2001 => println!("2000 or 2001"), // Multiple matches
        1900..1994 => println!("from 1900 to 1994"), // Range match
        _ => println!("Something else"), // Default case
    }

    println!(
        "Hours played: {}",
        if hoursPlayed > 100 {
            "You are a hardcore gamer"
        } else {
            "You are a casual gamer"
        });
}



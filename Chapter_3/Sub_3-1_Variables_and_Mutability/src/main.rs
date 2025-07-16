const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours is {THREE_HOURS_IN_SECONDS} in seconds");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces_valid = "    ";
    let spaces_valid = spaces_valid.len();

    //let mut spaces_invalid = "   ";
    //spaces_invalid = spaces_invalid.len();
}

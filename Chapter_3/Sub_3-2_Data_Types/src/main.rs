fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    println!("The value of tup.1 is: {}", tup.1);
}

pub fn main() {
    println!("Hello, world!");

    let my_str = String::from("this is a string in the stack");
    let shift: i32 = 1;

    for c in my_str.chars() {
        print!("{} ", (c as i32) + shift);
    }

    println!("modulo of negative num: {}", -5 % 2_i64);
}

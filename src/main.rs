mod chars;
mod ownership;
mod slices;
mod structures;
mod enumerations;
mod modularity;
mod generics;
mod serde;
mod display;
mod closures;
mod concurrency;

fn main() {
    // chars::main();
    // ownership::main();
    // slices::main();
    // structures::main();
    // enumerations::main();
    // modularity::main();
    // generics::main();
    // serde::main();
    // display::main();
    // closures::main();
    // concurrency::main();

    let mut x = 0;
    let y = &mut x;
    *y = 1;
    dbg!(y);
    dbg!(x);


    #[derive(Debug, Clone)]
    struct Data {
        a: i32,
        b: String,
    }
    let s = Data { a: 34, b: String::from("hello")};
    let t = s.clone();

    dbg!(s);
    dbg!(t);

}
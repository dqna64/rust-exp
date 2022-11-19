mod bind_closure_to_var {
    pub fn main() {
        let xor = |a, b| a != b;
        // closure type inferred
        dbg!(xor(1, 0));
        dbg!(xor(0, 1));
        dbg!(xor(0, 0));
        dbg!(xor(1, 1));
    }
}

mod mutable_borrow {

    pub fn main() {
        let mut quote = String::from("History is written by the victors");
        println!("Before defining closure: {:?}", quote);
    
        let mut finish_quote = || quote.insert_str(11, "not ");
        // Cannot borrow or take ownership of `quote` between closure definition
        // and closure call, it has been mutably borrowed!
        finish_quote();

        println!("After calling closure: {:?}", quote);
    }
}


pub fn main() {
    bind_closure_to_var::main();
    mutable_borrow::main();
}
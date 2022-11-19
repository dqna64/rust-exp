mod bind_closure_to_var {
    pub fn main() {
        println!("\n=== Bind closure to var ===");
        let xor = |a, b| a != b;
        // closure type inferred
        dbg!(xor(1, 0));
        dbg!(xor(0, 1));
        dbg!(xor(0, 0));
        dbg!(xor(1, 1));
    }
}

mod capture_by_ownership {
    /// Closure takes ownership of some resource from outside scope. Compiler
    /// automatically detects this and makes the closure implement FnOnce.
    pub fn main() {
        println!("\n=== Capture by ownership ===");
        let mut my_jewellery = String::from("I want some jewellery");
        let ring = String::from("One ring to rule them all");
        println!("Before defining closure: {:?}", my_jewellery);
        
        let obtain_jewellery = || my_jewellery = ring;
        obtain_jewellery();
        // `ring` was moved into closure, can no longer use `ring`
        // dbg!(ring);

        println!("After calling closure: {:?}", my_jewellery);
    }
}

mod capture_by_mutable_borrow {
    /// Closure borrows mutably some resource from outside scope. Compiler
    /// automatically detects this and makes the closure implement FnMut.
    pub fn main() {
        println!("\n=== Capture by mutable borrow ===");
        let mut quote = String::from("History is written by the victors");
        println!("Before defining closure: {:?}", quote);
    
        let mut finish_quote = || quote.insert_str(11, "not ");
        // Cannot borrow or take ownership of `quote` between closure definition
        // and closure call, it has been mutably borrowed!
        finish_quote();

        println!("After calling closure: {:?}", quote);
    }
}

mod capture_by_immutable_borrow {
    /// Closure borrows immutably some resource from outside scope. Compiler
    /// automatically detects this and makes the closure implement Fn.
    pub fn main() {
        println!("\n=== Capture by immutable borrow ===");
        let invariant = String::from("Entropy");
    
        let finish_quote = || println!("The only unvarying is {}", invariant);
        // Can only immutably borrow `invariant` between closure definition
        // and closure call, it has been immutably borrowed.
        println!("Read the value while immutably borrowed by closure: {}", invariant);

        finish_quote();

        // Take ownership of the value
        dbg!(invariant);

    }    
}    



mod move_into_closure {
    /// Closure captures some variable from outside scope by taking ownership.
    /// Commonly used for spawning threads that might outlive the variable's
    /// previous scope.
    /// Note that the closure type might not necessarily be `FnOnce`. The trait
    /// chosen for it depends on what it does with its captured values.
    use std::thread;
    pub fn main() {
        println!("\n=== Move into closure ===");

        let list = vec![1, 1, 2, 3, 5, 8];
        println!("Before defining closure: {:?}", list);
    
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();

        // Cannot use `list` after closure defined, it was moved into the closure.
        // dbg!(list);
    
    }
}

mod capture_single_struct_field {
    struct Supper {
        drink: String,
        snack: String,
    }
    
    pub fn main() {
        let mut my_supper = Supper {
            drink: String::from("Hot chocolate"),
            snack: String::from("Marshmellows"),
        };
        let have_a_sip = || println!("Sipping {}", my_supper.drink);
        my_supper.snack.push_str(" on toast");
        have_a_sip();
    }
}

mod closure_trait_bounds {
    /// From https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/
    fn call_Fn<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }
    
    fn call_FnMut<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }
    
    fn call_FnOnce<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    
    /// If `F` implements `Fn` then `&F` also
    /// If `F` implements `FnMut` then `&mut F` also
    /// So we can just pass references to those closures
    pub fn main() {
        let immut_val = String::from("immut");
        let Fn_closure = || {
            println!("Len: {}", immut_val.len());
        };
    
        let mut mut_val = String::from("mut");
        let mut FnMut_closure = || {
            mut_val.push_str("-new");
        };
    
        let value = String::from("value");
        let FnOnce_closure = || {
            let _moved_value = value;
        };
    
        call_Fn(&Fn_closure);
        call_FnMut(&Fn_closure);
        call_FnOnce(&Fn_closure);
    
        // call_fn(FnMut_closure); //error: FnMut_closure implements `FnMut`, not `Fn`
        call_FnMut(&mut FnMut_closure);
        call_FnOnce(&mut FnMut_closure);
    
        // call_fn(FnOnce_closure); //error: FnOnce_closure implements `FnOnce`, not `Fn`
        // call_fn_mut(FnOnce_closure); //error: FnOnce_closure implements `FnOnce`, not `FnMut`
        call_FnOnce(FnOnce_closure);
    }
}

pub fn main() {
    bind_closure_to_var::main();
    capture_by_ownership::main();
    capture_by_immutable_borrow::main();
    capture_by_mutable_borrow::main();
    move_into_closure::main();
    capture_single_struct_field::main();
    closure_trait_bounds::main();
}
mod structs {
    #[derive(Debug)]
    struct Particle {
        name: String,
        mass: f64,
        charge: f64,
        spin: f64,
    }

    pub fn main() {
        let mut my_up_quark = Particle {
            name: String::from("some up quark"),
            mass: 2.2,
            charge: 0.667,
            spin: 0.333,
        };
        
        my_up_quark.name = String::from("Up Quark!");

        // only fields with Copy trait borrowed, my_up_quark still valid
        let my_down_quark = Particle {
            name: String::from("some down quark"),
            ..my_up_quark
        };

        println!("{my_up_quark:?}"); // still valid

        // fields with Drop trait borrowed, my_up_quark no longer valid
        let my_strange_quark = Particle {
            mass: 1.4,
            ..my_up_quark
        };

        // borrow of partially moved value: `my_up_quark`
        // println!("{my_up_quark:?}");

    }
}

mod tuple_structs {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Feeling(String, i32);

    pub fn main() {
        let my_fav_col = Color(53, 209, 240);
        let red = my_fav_col.0;
        println!("{red}");

        let my_feeling: Feeling = Feeling(String::from("djfklsa"), 45);
        let what_feeling = my_feeling.0;
        println!("{what_feeling}");
        // field with Drop trait borrowed, my_feeling no longer valid
        // println!("{my_feeling:?}");

    }
}

mod unit_like_structs {
    #[derive(Debug)]
    struct Unit;

    pub fn main() {
        let u1 = Unit;
        println!("{u1:?}");
        
        let u2 = u1;
        println!("{u2:?}");

    }
}

mod struct_ownership_lifetimes {
    struct Item;
    // struct Container needs to specify lifetime of the thing
    // that it's field references.
    struct Container<'a>(&'a Item);

    pub fn main() {
        let i = Item;
        let c = Container(&i);
        
    }
}



fn main() {

    structs::main();
    tuple_structs::main();

    
}

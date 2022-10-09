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

mod struct_associated_fns {
    #[derive(Debug)]
    enum Condition {
        new,
        used,
    }
    #[derive(Debug)]
    struct Car {
        brand: String,
        condition: Condition,
    }

    impl Car {
        // In impl block, Self is an alias for whatever comes after impl
        fn new(brand: String) -> Self {
            Self {
                brand,
                condition: Condition::new,
            }
        }
        fn used(brand: String) -> Self {
            Self {
                brand,
                condition: Condition::used,
            }
        }
    }

    pub fn main() {
        let my_new_car = Car::new(String::from("Volswagen"));
        dbg!(my_new_car);
        let my_used_car = Car::used(String::from("GM"));
        dbg!(my_used_car);

    }
}

mod struct_methods {
    #[derive(Debug)]
    struct Egg {
        typ: String,
        mass: i32,
    }

    impl Egg {
        fn scramble(&mut self) { //  Short for `&mut self: &mut Self`
            self.typ.push_str(" scrambed");
            self.mass -= 21;
        }
    }

    pub fn main() {
        let mut my_egg = Egg {
            typ: String::from("free range"),
            mass: 243,
        };

        my_egg.scramble();
        dbg!(my_egg);
    }
}


pub fn main() {

    structs::main();
    tuple_structs::main();
    
    struct_methods::main();
    struct_associated_fns::main();
    struct_methods::main();
    
}

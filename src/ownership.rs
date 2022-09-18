pub fn main() {
    {
        // Copying primitive types like integers
        let i = 4732;
        let j = i;
        println!("{i:?} {j:?}");
    }

    {
        // Copying stack strings (string literals)
        let i = "string literal on stack";
        let j = i;
        println!("{i:?} {j:?}");
    }

    {
        // Copying heap strings
        let s1 = String::from("Good morning");
        let s2 = s1;
        // Value of s1 moved to s2
        // println!("{s1:?}");
        println!("{s2:?}");
    }

    {
        let mut s = String::from("String in some scope");

        s.make_ascii_uppercase();
        s.push_str(" with new additions");
        
        {
            println!("Access string s in nested scope: {s}");
        }
        // drop implicitly called on s when going out of scope
    }

    {
        let obtained_str = gives_ownership();
        println!("Got string {obtained_str} from gives_ownership fn");
    }

    {
        let size: usize = 432;
        let size_cpy = size;
        println!("{size:?} {size_cpy:?}");
    }

    
}

fn gives_ownership() -> String {
    let new_str = String::from("New from from fn");
    return new_str;
}
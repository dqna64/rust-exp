pub fn main() {
    {
        // Copying integers (implements Copy)
        let i = 4732;
        let j = i;
        println!("{i:?} {j:?}");
    }

    {
        // Copying stack strings (string literals) (implements Copy)
        let i = "string literal on stack";
        let j = i;
        println!("{i:?} {j:?}");
    }

    {
        // Moving heap strings
        let s1 = String::from("Good morning");
        let s2 = s1;
        // Value of s1 moved to s2
        // println!("{s1:?}"); // Move occurs here
        println!("{s2:?}");
    }

    {
        // Clone heap strings (implements Clone)
        let s1 = String::from("Initial copy");
        let s2 = s1.clone();
        // Value of s1 cloned to s2
        println!("{s1:?}");
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

    {
        let mut lis = vec![1,2,3];
        lis.push(4);
        let lis_ref = &mut lis;
        lis_ref.push(5);
        println!("{lis_ref:?}");
    }

    {
        let lis = vec![1,2,3];
        let mut sum = 0;
        lis.iter().for_each(|x| sum += x);
    }

    
}

fn gives_ownership() -> String {
    let new_str = String::from("New from from fn");
    return new_str;
}
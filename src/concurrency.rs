mod scope_immutable_borrow {
    /// Thread guaranteed to end before main1 fn, but compiler still complains.
    /// Not smart enough. Need to capture by `move`, or use scope.
    /*
    fn main1() {
        let a = String::from("black friday");
        let handler = std::thread::spawn(|| { // Err: closure may outlive fn...
            dbg!(&a);
        });
        handler.join().unwrap();
        dbg!(&a);
    }
    */

    pub fn main2() {
        println!("\n=== Thread makes immutable borrow ===");
        let a = String::from("cyber monday");

        std::thread::scope(|s| {
            s.spawn(|| {
                dbg!(&a);
            });
        });

        // Thread s has been joined. We can use our variable again.

        dbg!(&a);
    }
}

mod share_data_among_threads {
    use std::sync::Arc;

    pub fn main() {
        println!("\n=== Share data among threads ===");
        let shared = Arc::new(String::from("pudding"));
        let kids = vec!["Tim", "Tam", "Tom"];

        let mut threads = Vec::new();

        for kid in kids {
            let shared_clone = shared.clone();
            threads.push(std::thread::spawn(move || {
                println!(
                    "Kid named {} is sharing the {}",
                    kid, shared_clone
                );
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        // You can still use the original `shared`
        dbg!(shared);

    }
}


pub fn main() {
    scope_immutable_borrow::main2();
    share_data_among_threads::main();
}

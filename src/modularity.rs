mod society {
    pub mod executives {
        pub fn election() {
            println!("Holding election");
        }

        pub fn poll() {
            println!("Holding poll");
        }
    }

    mod directors {
        fn recruit() {
            println!("Recruit directors")
        }

        mod education {
            fn educate() {
                println!("Educate ppl");
            }
        }
    }
}

mod relative_paths_with_super {
    mod shell {
        pub fn crack(source: String) {
            println!("Cracking egg shell from {source}");
        }
        pub mod albumen {
            pub fn unleash_myself() {
                super::crack(String::from("albumen"));
            }
            pub mod yolk {
                pub fn unleash_myself() {
                    super::super::crack(String::from("yolk"));
                }
            }
        }
    }

    pub fn main() {
        shell::crack(String::from("main fn"));
        shell::albumen::unleash_myself();
        shell::albumen::yolk::unleash_myself();
    }
}

use crate::enumerations::enum_values;

pub fn main() {
    // Absolute path
    crate::modularity::society::executives::poll();

    // Relative path
    society::executives::election();

    // module imported from another crate
    enum_values::main();

    relative_paths_with_super::main();


}
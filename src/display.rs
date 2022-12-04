


mod display_trait {
    trait Machine: std::fmt::Display + std::fmt::Debug {
        fn cycle(&mut self);
    }
    
    #[derive(Debug)]
    struct Computer {
        cpu: i32,
        ram: f64,
        submachine: Option<Box<dyn Machine>>,
    }
    impl Machine for Computer {
        fn cycle(&mut self) {
            println!("Cycling");
        }
    }
    impl std::fmt::Display for Computer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}\n", self.cpu, self.ram)?;
            match &self.submachine {
                Some(machine) => write!(f, "{:indent$}{}\n", "", machine, indent=2),
                None => write!(f, "{:indent$}{}\n", "", String::from("No submachine"), indent=2),
            }?;
            Ok(())
        }
        
    }

    pub fn main() {
        let xps13 = Computer {
            cpu: 34,
            ram: 634.23,
            submachine: None,
        };
        
        let xps15 = xps13;

        dbg!(&xps15);
        println!("{}", xps15);

        let recursive_computer = Computer {
            cpu: 432,
            ram: 9348.4324,
            submachine: Some(Box::new(
                Computer {
                    cpu: 34,
                    ram: 634.23,
                    submachine: None,
                }
            )),
        };

        println!("{}", recursive_computer);
    }
}

mod display_trait_with_indenter {
    use indent_write::fmt::IndentWriter;
    use std::fmt::Write;
    trait Machine: std::fmt::Display + std::fmt::Debug {
        fn cycle(&mut self);
    }
    
    #[derive(Debug)]
    struct Computer {
        cpu: i32,
        ram: f64,
        submachine: Option<Box<dyn Machine>>,
    }
    impl Machine for Computer {
        fn cycle(&mut self) {
            println!("Cycling");
        }
    }
    impl std::fmt::Display for Computer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            
            let mut buffer = String::new();
            let mut writer = IndentWriter::new_skip_initial("\t", &mut buffer);
            
            writeln!(writer, "{} {}", self.cpu, self.ram);
            match &self.submachine {
                Some(machine) => writeln!(writer, "{}", machine).unwrap(),
                None => writeln!(writer, "{}", String::from("No submachine")).unwrap(),
            };
            write!(f, "{}", buffer);
            Ok(())
        }
        
    }

    pub fn main() {
        let recursive_computer = Computer {
            cpu: 432,
            ram: 9348.4324,
            submachine: Some(Box::new(
                Computer {
                    cpu: 34,
                    ram: 634.23,
                    submachine: None,
                }
            )),
        };

        println!("{}", recursive_computer);
    }
}


pub fn main() {
    display_trait_with_indenter::main();
}
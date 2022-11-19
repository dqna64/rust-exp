mod copy_trait {
    #[derive(Copy)]
    struct Computer {
        cpu: i32,
        ram: f64,
    }

    pub fn main() {
        let xps13 = Computer{
            cpu: 432,
            ram: 9348.4324,
        };
        
        let xps15 = xps13;

        // xps13 still valid
        dbg!(xps13);
        dbg!(xps15);
    }
}

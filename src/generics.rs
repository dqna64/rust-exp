mod generics_in_fns {
    fn get_smallest<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
        if list.len() == 0 {
            return None
        }
        let mut smallest = &list[0];
        for n in list {
            if n < smallest {
                smallest = n
            }
        }
        return Some(smallest);
    }

pub fn main() {
        let nums = vec![99, 57, 26, 65, 17, 49];
        let smallest_num = get_smallest::<i32>(&nums);
        dbg!(smallest_num);

        let chars = vec!['z', 't', 'y', 'm', 'q', 'f', 'h'];
        let smallest_char = get_smallest::<char>(&chars);
        dbg!(smallest_char);
    }
}

pub fn main() {
    generics_in_fns::main();
}
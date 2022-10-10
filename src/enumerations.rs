
mod enum_ {
    #[derive(Debug)]
    enum Status {
        Todo,
        Doing,
        Done,
    }

    pub fn main() {
        let exercise = Status::Todo;
        dbg!(&exercise);

        dbg!(status_update(&exercise));
    }

    fn status_update(s: &Status) -> &str {
        match s {
            Status::Todo => "Just do it",
            Status::Doing => "Finish it",
            Status::Done => "Nice work",

        }
    }
}

mod enum_values {
    /**
     * You can attch data to each variant of an enum.
     * The name of each enum variant becomes a function that contructs
     * an instance of the enum and returns it (an automatically
     * defined constructor).
     */

    #[derive(Debug)]
    enum NetworkLayer {
        Application(String),
        Transport(String),
        Network(String),
        Link(String),
        Physical(String),
    }

    pub fn main() {
        let my_desktop_app = NetworkLayer::Application(String::from("Slack"));
        dbg!(my_desktop_app);
    }
}

mod enum_values_diff_args {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub fn main() {
        let cse = IpAddr::V4(129, 94, 242, 117);
        let csev6 = IpAddr::V6(String::from("ed98:901e:32a1:78a2:932e:c37b:f92a:9fb2"));
        dbg!(cse, csev6);

    }
        
}

mod enum_vs_struct {
    // todo!("do this");
}

mod enum_impl {
    // todo!("do this");
}

mod option_enum {
    // todo!("do this");
}

mod enum_pattern_matching {
    enum TrainModel {
        Waratah,
        Millenium,
        Tangara,
    }
    enum Transport {
        Bus,
        Lightrail,
        Train(TrainModel),
    }

    pub fn main() {
        let night_ride = Transport::Train(TrainModel::Millenium);
        let what = match night_ride {
            Transport::Bus => "By bus",
            Transport::Lightrail => "By tram",
            Transport::Train(model) => match model {
                TrainModel::Waratah => "By waratah train",
                TrainModel::Millenium => "By millenium train",
                TrainModel::Tangara => "By tangara train",
            }
        };
        dbg!(what);
    }
}

pub fn main() {
    enum_::main();
    enum_values::main();
    enum_values_diff_args::main();
    enum_pattern_matching::main();

}
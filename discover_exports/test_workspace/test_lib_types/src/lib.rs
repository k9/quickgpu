pub struct BigType {
    pub a: u32,
}

impl Default for BigType {
    fn default() -> Self {
        Self { a: 123 }
    }
}

pub mod counters {
    pub struct CounterA {}
    pub struct CounterB {}
    pub struct CounterC {}
    pub struct CounterD {
        pub d: CounterC,
    }

    mod inner {
        mod inner2 {
            pub struct CounterC {}
        }

        mod inner3 {
            pub use super::inner2::CounterC;
        }

        pub use inner3::CounterC;
    }

    pub fn abc() {
        inner::CounterC {};
    }
}

pub use counters::*;

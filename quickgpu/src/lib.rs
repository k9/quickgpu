pub mod builders;

pub mod aa {
    pub struct A {
        pub b: f64,
    }
}

pub mod bb {
    pub use crate::aa::*;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

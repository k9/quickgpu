pub mod builders;
pub mod nested;

pub trait Nested<T> {
    fn unnest(self) -> T;
}

impl Nested<u32> for u32 {
    fn unnest(self) -> u32 {
        self
    }
}

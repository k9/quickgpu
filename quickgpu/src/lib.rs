pub mod builders;
pub mod nested;

pub trait Nested<T> {
    fn unnest(self) -> T;
}

mod builders;
pub use builders::*;

pub trait Nested<T> {
    fn unnest(self) -> T;
}

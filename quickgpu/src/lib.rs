pub mod builders;
pub mod nested;

pub trait Nested<T>: Default {
    fn unnest(self) -> T;
}

impl<T, N: Nested<T>> Nested<Option<T>> for Option<N> {
    fn unnest(self) -> Option<T> {
        self.map(|t| t.unnest())
    }
}

impl<T, N: Nested<T>> Nested<Vec<T>> for Vec<N> {
    fn unnest(self) -> Vec<T> {
        self.into_iter().map(|t| t.unnest()).collect::<Vec<_>>()
    }
}

impl<'a, T> Nested<&'a [T]> for &'a [T] {
    fn unnest(self) -> &'a [T] {
        self
    }
}

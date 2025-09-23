use serde::{Deserialize, Serialize};

pub trait MaybeDefault<T> {
    fn maybe_default() -> Option<T>;
}

// The generic implementation
impl<T> MaybeDefault<T> for T {
    default fn maybe_default() -> Option<T> {
        None
    }
}

impl<T: Default> MaybeDefault<T> for T {
    default fn maybe_default() -> Option<T> {
        Some(T::default())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DummyStruct;

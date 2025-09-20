#![feature(specialization)]

mod maybe_default;
use std::collections::HashMap;
use wgpu::*;

use crate::maybe_default::MaybeDefault;

pub fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(
        "MultisampleState".to_string(),
        format!("{:?}", MultisampleState::maybe_default()),
    );

    dbg!(map);
}

use bon::builder;
use wgpu::util::*;
use wgpu::*;

#[builder(state_mod(vis = "pub(crate)"))]
pub fn abc<T, U, 'a>(z: &'a T, zz: U) -> Abc<T, U, 'a> {
    Abc { z, zz }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn def<T, U, 'a>(z: &'a T, zz: U) -> Def<T, U, 'a> {
    Def { z, zz }
}

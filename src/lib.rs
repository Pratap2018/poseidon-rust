extern crate ff;

mod constants;
pub mod hash;
pub mod poseidon;

pub use ff::PrimeField;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POSEIDON: poseidon::Poseidon = poseidon::Poseidon::default();
}

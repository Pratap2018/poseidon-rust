use crate::ff::Field;
use crate::ff::{PrimeField, PrimeFieldRepr};
use crate::poseidon::{Fr, FrRepr};
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone)]
pub struct PoseidonHash(Fr);

impl PoseidonHash {
    pub fn bytes_be(&self) -> Vec<u8> {
        let repr = self.0.into_repr();
        let required_length = repr.as_ref().len() * 8;
        let mut buf: Vec<u8> = Vec::with_capacity(required_length);
        repr.write_be(&mut buf).unwrap();
        buf
    }

    pub fn bytes_le(&self) -> Vec<u8> {
        let repr = self.0.into_repr();
        let required_length = repr.as_ref().len() * 8;
        let mut buf: Vec<u8> = Vec::with_capacity(required_length);
        repr.write_le(&mut buf).unwrap();
        buf
    }

    pub fn hex(&self) -> String {
        let bytes = self.bytes_be();
        hex::encode(bytes)
    }

    pub fn bigint(&self) -> BigUint {
        let bytes = self.bytes_be();
        BigUint::from_bytes_be(bytes.as_slice())
    }

    pub fn string(&self) -> String {
        let bigint = self.bigint();
        bigint.to_string()
    }

    pub fn fr(&self) -> Fr {
        self.0
    }
}

impl Default for PoseidonHash {
    fn default() -> Self {
        Self(Fr::zero())
    }
}

impl From<Fr> for PoseidonHash {
    fn from(value: Fr) -> Self {
        PoseidonHash(value)
    }
}

impl From<&[u8]> for PoseidonHash {
    fn from(value: &[u8]) -> Self {
        let bigint = BigUint::from_bytes_be(value);
        PoseidonHash(Fr::from_str(&bigint.to_string()).unwrap_or(Fr::default()))
    }
}

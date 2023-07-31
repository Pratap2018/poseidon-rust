use num_bigint::BigInt;

use crate::ff::{PrimeField, PrimeFieldRepr};
use crate::poseidon::Fr;

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

    pub fn bigint(&self) -> BigInt {
        let bytes = self.bytes_be();
        BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes.as_slice())
    }

    pub fn string(&self) -> String {
        let bigint = self.bigint();
        bigint.to_string()
    }

    pub fn fr(&self) -> Fr {
        self.0
    }
}

impl From<Fr> for PoseidonHash {
    fn from(value: Fr) -> Self {
        PoseidonHash(value)
    }
}

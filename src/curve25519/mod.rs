use std::convert::TryInto;

use rand::{CryptoRng, Rng, RngCore};

use crate::{curve25519::scalar::Scalar, sha512};

mod arithmetic;
mod field;
mod point;
mod scalar;

const PUBLIC_KEY_SIZE: usize = 32;

pub struct PublicKey {
    pub bytes: [u8; PUBLIC_KEY_SIZE],
}

const PRIVATE_KEY_SIZE: usize = 32;

pub struct PrivateKey {
    pub bytes: [u8; PRIVATE_KEY_SIZE],
}

impl PrivateKey {
    fn derive_public_key(&self) -> PublicKey {
        let hash = sha512::hash(&self.bytes);
        let scalar = Scalar::clamped(hash[..32].try_into().unwrap());
        println!("scalar: {:X?}", scalar);
        PublicKey {
            bytes: (&point::B * scalar).into(),
        }
    }
}

pub fn gen_keypair<R: RngCore + CryptoRng>(rng: &mut R) -> (PublicKey, PrivateKey) {
    let mut private = PrivateKey { bytes: [0u8; 32] };
    rng.fill_bytes(&mut private.bytes);
    (private.derive_public_key(), private)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_derivation_examples() {
        let mut private = PrivateKey {
            bytes: [0; 32]
        };
        hex::decode_to_slice(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
            &mut private.bytes
        ).unwrap();
        let mut expected = [0; 32];
        hex::decode_to_slice(
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            &mut expected
        ).unwrap();
        let public = private.derive_public_key();
        assert_eq!(public.bytes, expected);
    }
}

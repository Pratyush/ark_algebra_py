use ark_bls12_381::{Fr, G1Projective, G2Projective};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use num_traits::identities::{One, Zero};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const SCALAR_SIZE: usize = 32;

crate::monomorphize_field!(Scalar, Fr, SCALAR_SIZE);
crate::monomorphize_point!(G1, G1Projective, Scalar, 48);
crate::monomorphize_point!(G2, G2Projective, Scalar, 96);

crate::monomorphize_pairing!(Pairing, ark_bls12_381::Bls12_381, G1, G2);

crate::monomorphize_poly!(Fr, Scalar);

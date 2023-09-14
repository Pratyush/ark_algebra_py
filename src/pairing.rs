#[macro_export]
macro_rules! monomorphize_pairing {
    ($struct: ident, $inner: ty, $g1: ty, $g2: ty) => {
        use ark_ec::pairing::{Pairing as _, PairingOutput as POutput};

        #[derive(Copy, Clone)]
        #[pyo3::pyclass]
        pub struct PairingOutput(POutput<$inner>);

        #[pyo3::pymethods]
        impl PairingOutput {
            #[new]
            pub fn generator() -> Self {
                use ark_ec::Group;
                Self(POutput::generator())
            }

            #[staticmethod]
            pub fn one() -> Self {
                Self(POutput::<$inner>::zero())
            }

            // Overriding operators
            fn __mul__(&self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }

            pub fn inverse(&self) -> Self {
                Self(-self.0)
            }

            pub fn square(&self) -> Self {
                use ark_ec::Group;
                Self(self.0.double())
            }

            fn __str__(&self) -> pyo3::PyResult<String> {
                Ok(format!("{}", self.0 .0))
            }

            fn __richcmp__(&self, other: Self, op: pyclass::CompareOp) -> pyo3::PyResult<bool> {
                match op {
                    pyclass::CompareOp::Eq => Ok(self.0 == other.0),
                    pyclass::CompareOp::Ne => Ok(self.0 != other.0),
                    _ => Err(pyo3::exceptions::PyValueError::new_err(
                        "comparison operator not implemented".to_owned(),
                    )),
                }
            }
        }

        #[derive(Copy, Clone)]
        #[pyclass]
        pub struct $struct($inner);

        #[pymethods]
        impl $struct {
            #[staticmethod]
            fn multi_pairing(py: Python, g1s: Vec<$g1>, g2s: Vec<$g2>) -> PairingOutput {
                py.allow_threads(|| {
                    let g1_inner: Vec<ark_bls12_381::G1Affine> =
                        g1s.into_par_iter().map(|g1| g1.0.to_affine()).collect();
                    let g2_inner: Vec<ark_bls12_381::G2Affine> =
                        g2s.into_par_iter().map(|g2| g2.0.to_affine()).collect();
                    PairingOutput(<$inner>::multi_pairing(g1_inner, g2_inner))
                })
            }

            #[staticmethod]
            fn pairing(py: Python, g1: $g1, g2: $g2) -> PairingOutput {
                py.allow_threads(|| {
                    PairingOutput(<$inner>::pairing(g1.0.to_affine(), g2.0.to_affine()))
                })
            }
        }
    };
}

// pub struct Pair<P: Pairing>(PhantomData<P>);

// #[derive(Copy, Clone)]
// #[pyclass]
// pub struct GT(PairingOutput<ark_bls12_381::Bls12_381>);

// #[pymethods]
// impl GT {
//     #[new]
//     fn generator() -> GT {
//         GT(PairingOutput::generator())
//     }

//     #[staticmethod]
//     fn one() -> GT {
//         GT(PairingOutput::zero())
//     }

//     // Overriding operators
//     fn __mul__(&self, rhs: GT) -> GT {
//         // The group operation is multiplication of the underlying field, but
//         // the group is written additively
//         GT(self.0 + rhs.0)
//     }

//     fn inverse(&self) -> GT {
//         // The group operation is multiplication of the underlying field, but
//         // the group is written additively
//         GT(-self.0)
//     }
//     fn __str__(&self) -> PyResult<String> {
//         let mut bytes = Vec::new();
//         self.0
//             .serialize_compressed(&mut bytes)
//             .map_err(serialisation_error_to_py_err)?;
//         Ok(hex::encode(bytes))
//     }
//     fn __richcmp__(&self, other: GT, op: pyclass::CompareOp) -> PyResult<bool> {
//         match op {
//             pyclass::CompareOp::Eq => Ok(self.0 == other.0),
//             pyclass::CompareOp::Ne => Ok(self.0 != other.0),
//             _ => Err(exceptions::PyValueError::new_err(
//                 "comparison operator not implemented".to_owned(),
//             )),
//         }
//     }
// }

// #[macro_export]
// macro_rules! monomorphize_field {
//     ($struct: ident, $inner: ty, $COMPRESSED_SIZE: expr) => {
//         use ark_ff::Field;
//         use pyo3::{PyResult, pyclass, exceptions};

//         use crate::utils::serialisation_error_to_py_err;

//         #[derive(Copy, Clone)]
//         #[pyclass]
//         pub struct $struct($inner);

//         #[pymethods]
//         impl $struct {
//             fn new(integer: u128) -> Self {
//                 Self(<$inner>::from(integer))
//             }

//             fn zero() -> Self {
//                 Self(<$inner>::zero())
//             }

//             fn one() -> Self {
//                 Self(<$inner>::one())
//             }

//             // Overriding operators
//             fn __add__(&self, rhs: Self) -> Self {
//                 Self(self.0 + rhs.0)
//             }

//             fn __sub__(&self, rhs: Self) -> Self {
//                 Self(self.0 - rhs.0)
//             }

//             fn __mul__(&self, rhs: Self) -> Self {
//                 Self(self.0 * rhs.0)
//             }

//             fn __neg__(&self) -> Self {
//                 Self(-self.0)
//             }

//             fn __str__(&self) -> PyResult<String> {
//                 return Ok(hex::encode(self.to_le_bytes()?));
//             }

//             fn __richcmp__(&self, other: Self, op: pyclass::CompareOp) -> PyResult<bool> {
//                 match op {
//                     pyclass::CompareOp::Eq => Ok(self.0 == other.0),
//                     pyclass::CompareOp::Ne => Ok(self.0 != other.0),
//                     _ => Err(exceptions::PyValueError::new_err(
//                         "comparison operator not implemented".to_owned(),
//                     )),
//                 }
//             }

//             fn square(&self) -> Self {
//                 Self(self.0.square())
//             }

//             fn double(&self) -> Self {
//                 Self(self.0.double())
//             }

//             fn inverse(&self) -> Self {
//                 Self(self.0.inverse().unwrap_or_default())
//             }

//             fn batch_inverse(elems: Vec<Self>) -> Vec<Self> {
//                 let mut elems = elems.into_iter().map(|e| e.0).collect::<Vec<_>>();
//                 ark_ff::batch_inversion(&mut elems);
//                 elems.into_iter().map(Self).collect()
//             }

//             fn is_zero(&self) -> bool {
//                 self.0.is_zero()
//             }

//             fn is_one(&self) -> bool {
//                 self.0.is_one()
//             }

//             fn to_le_bytes<const N: usize>(&self) -> PyResult<[u8; N]> {
//                 let mut bytes = [0u8; N];
//                 self.0
//                     .serialize_compressed(&mut bytes[..])
//                     .map_err(serialisation_error_to_py_err)
//                     .map(|_| bytes)
//             }

//             fn from_le_bytes<const N: usize>(bytes: [u8; N]) -> PyResult<Self> {
//                 <$inner>::deserialize_compressed(&bytes[..])
//                     .map_err(serialisation_error_to_py_err).map(Self)
//             }
//         }
//     }
// }

#[macro_export]
macro_rules! monomorphize_pairing {
    ($struct: ident, $inner: ty, $g1: ty, $g2: ty) => {
        use ark_ec::pairing::{Pairing as _, PairingOutput as POutput};

        #[derive(Copy, Clone)]
        #[pyo3::pyclass]
        pub struct GT(POutput<$inner>);

        #[pyo3::pymethods]
        impl GT {
            /// Returns the generator of the target group.
            #[new]
            pub fn generator() -> Self {
                use ark_ec::Group;
                Self(POutput::generator())
            }

            /// Returns the identity of the target group.
            #[staticmethod]
            pub fn one() -> Self {
                Self(POutput::<$inner>::zero())
            }

            // Overriding operators
            /// Multiply two elements of `GT`.
            fn __mul__(&self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }

            /// Exponentiate an element of `GT` by an integer.
            fn __pow__(&self, other: i128, _modulo: Option<u128>) -> Self {
                Self(self.0 * Scalar(other.into()).0)
            }

            /// Returns the inverse of an element of `self`.
            pub fn inverse(&self) -> Self {
                Self(-self.0)
            }

            /// Returns the square of an element of `self`
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
            /// Computes the product of the point-wise pairings of the
            /// elements of `g1s` and `g2s`.
            #[staticmethod]
            fn multi_pairing(py: Python, g1s: Vec<$g1>, g2s: Vec<$g2>) -> GT {
                py.allow_threads(|| {
                    let g1_inner: Vec<ark_bls12_381::G1Affine> =
                        g1s.into_par_iter().map(|g1| g1.0.to_affine()).collect();
                    let g2_inner: Vec<ark_bls12_381::G2Affine> =
                        g2s.into_par_iter().map(|g2| g2.0.to_affine()).collect();
                    GT(<$inner>::multi_pairing(g1_inner, g2_inner))
                })
            }

            /// Computes the pairing `e(g1, g2)`.
            #[staticmethod]
            fn pairing(py: Python, g1: $g1, g2: $g2) -> GT {
                py.allow_threads(|| {
                    GT(<$inner>::pairing(g1.0.to_affine(), g2.0.to_affine()))
                })
            }
        }
    };
}
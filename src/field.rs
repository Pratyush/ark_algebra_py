#[macro_export]
macro_rules! monomorphize_field {
    ($struct: ident, $inner: ty, $COMPRESSED_SIZE: expr) => {
        use ark_ff::Field;

        use pyo3::{exceptions, pyclass, pymethods, PyResult, Python};

        use $crate::utils::serialisation_error_to_py_err;

        #[derive(Copy, Clone)]
        #[pyclass]
        pub struct $struct($inner);

        #[pymethods]
        impl $struct {
            #[new]
            fn new(integer: u128) -> Self {
                Self(<$inner>::from(integer))
            }

            #[staticmethod]
            fn zero() -> Self {
                Self(<$inner>::zero())
            }

            #[staticmethod]
            fn one() -> Self {
                Self(<$inner>::one())
            }

            // Overriding operators
            fn __add__(&self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }

            fn __sub__(&self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }

            fn __mul__(&self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }

            fn __neg__(&self) -> Self {
                Self(-self.0)
            }

            fn __pow__(&self, other: u64, _modulo: Option<u128>) -> Self {
                Self(self.0.pow([other]))
            }

            fn __str__(&self) -> PyResult<String> {
                if self.0.is_zero() {
                    Ok("0".to_owned())
                } else {
                    Ok(format!("{}", self.0))
                }
            }

            fn __richcmp__(&self, other: Self, op: pyclass::CompareOp) -> PyResult<bool> {
                match op {
                    pyclass::CompareOp::Eq => Ok(self.0 == other.0),
                    pyclass::CompareOp::Ne => Ok(self.0 != other.0),
                    _ => Err(exceptions::PyValueError::new_err(
                        "comparison operator not implemented".to_owned(),
                    )),
                }
            }

            fn square(&self) -> Self {
                Self(self.0.square())
            }

            fn double(&self) -> Self {
                Self(self.0.double())
            }

            fn inverse(&self) -> Self {
                Self(self.0.inverse().unwrap_or_default())
            }

            #[staticmethod]
            fn batch_inverse(elems: Vec<Self>) -> Vec<Self> {
                let mut elems = elems.into_iter().map(|e| e.0).collect::<Vec<_>>();
                ark_ff::batch_inversion(&mut elems);
                elems.into_iter().map(Self).collect()
            }

            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }

            fn is_one(&self) -> bool {
                self.0.is_one()
            }

            fn to_le_bytes(&self) -> PyResult<[u8; $COMPRESSED_SIZE]> {
                let mut bytes = [0u8; $COMPRESSED_SIZE];
                self.0
                    .serialize_compressed(&mut bytes[..])
                    .map_err(serialisation_error_to_py_err)
                    .map(|_| bytes)
            }

            #[staticmethod]
            fn from_le_bytes(bytes: [u8; $COMPRESSED_SIZE]) -> PyResult<Self> {
                <$inner>::deserialize_compressed(&bytes[..])
                    .map_err(serialisation_error_to_py_err)
                    .map(Self)
            }
        }
    };
}

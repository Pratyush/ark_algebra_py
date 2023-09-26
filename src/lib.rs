mod wrapper;
#[macro_use]
mod point;
mod field;
mod pairing;
mod polynomial;
pub(crate) mod utils;

use pyo3::prelude::*;
use wrapper::{Domain, Pairing, GT, Polynomial, Scalar, G1, G2};

/// A Python module implemented in Rust.
#[pymodule]
fn ark_algebra_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scalar>()?;
    m.add_class::<G1>()?;
    m.add_class::<G2>()?;
    m.add_class::<Pairing>()?;
    m.add_class::<GT>()?;
    m.add_class::<Polynomial>()?;
    m.add_class::<Domain>()?;

    Ok(())
}

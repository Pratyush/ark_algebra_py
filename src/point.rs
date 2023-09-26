use std::collections::BTreeMap;

use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use pyo3::{exceptions, pyclass, PyResult, Python};
use rayon::prelude::*;

use crate::utils::serialisation_error_to_py_err;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Point<G: CurveGroup> {
    Point(G),
    Affine(G::Affine),
}

impl<G: CurveGroup> Copy for Point<G> {}

impl<G: CurveGroup> Point<G> {
    pub fn generator() -> Self {
        Self::Affine(G::Affine::generator())
    }

    pub fn identity() -> Self {
        Self::Affine(G::Affine::zero())
    }

    pub fn as_affine(&self) -> Option<&G::Affine> {
        match self {
            Self::Affine(p) => Some(p),
            Self::Point(_) => None,
        }
    }

    pub fn to_affine(&self) -> G::Affine {
        match self {
            Self::Affine(p) => *p,
            Self::Point(p) => p.into_affine(),
        }
    }

    pub fn to_group(&self) -> G {
        match self {
            Self::Affine(p) => p.into_group(),
            Self::Point(p) => *p,
        }
    }

    pub fn as_point(&self) -> Option<&G> {
        match self {
            Self::Point(p) => Some(p),
            Self::Affine(_) => None,
        }
    }

    // Overriding operators
    pub fn __add__(&self, rhs: Self) -> Self {
        let result = match (*self, rhs) {
            (Point::Point(lhs), Point::Point(rhs)) => lhs + rhs,
            (Point::Point(lhs), Point::Affine(rhs)) => lhs + rhs,
            (Point::Affine(lhs), Point::Point(rhs)) => lhs + rhs,
            (Point::Affine(lhs), Point::Affine(rhs)) => lhs + rhs,
        };
        Self::Point(result)
    }

    pub fn __sub__(&self, rhs: Self) -> Self {
        let result = match (*self, rhs) {
            (Point::Point(lhs), Point::Point(rhs)) => lhs - rhs,
            (Point::Point(lhs), Point::Affine(rhs)) => lhs - rhs,
            (Point::Affine(lhs), Point::Point(rhs)) => lhs + (-rhs),
            (Point::Affine(lhs), Point::Affine(rhs)) => lhs + (-rhs.into_group()),
        };
        Self::Point(result)
    }

    pub fn __mul__(&self, rhs: G::ScalarField) -> Self {
        let result = match *self {
            Point::Point(lhs) => lhs * rhs,
            Point::Affine(lhs) => lhs * rhs,
        };
        Self::Point(result)
    }

    pub fn __neg__(&self) -> Self {
        match *self {
            Point::Point(lhs) => Self::Point(-lhs),
            Point::Affine(lhs) => Self::Point(-lhs.into_group()),
        }
    }

    pub fn double(&self) -> Self {
        match *self {
            Point::Point(lhs) => Self::Point(lhs.double()),
            Point::Affine(lhs) => Self::Point(lhs.into_group().double()),
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        return Ok(format!("{}", self.to_affine()));
    }

    pub fn __richcmp__(&self, other: Self, op: pyclass::CompareOp) -> PyResult<bool> {
        let other = other.to_group();
        let this = self.to_group();
        match op {
            pyclass::CompareOp::Eq => Ok(this == other),
            pyclass::CompareOp::Ne => Ok(this != other),
            _ => Err(exceptions::PyValueError::new_err(
                "comparison operator not implemented".to_owned(),
            )),
        }
    }

    pub fn to_compressed_bytes<const N: usize>(&self) -> PyResult<[u8; N]> {
        let mut bytes = [0u8; N];
        let result = match self {
            Point::Point(point) => point.serialize_compressed(&mut bytes[..]),
            Point::Affine(point) => point.serialize_compressed(&mut bytes[..]),
        };
        result.map_err(serialisation_error_to_py_err).map(|_| bytes)
    }

    pub fn from_compressed_bytes<const N: usize>(bytes: [u8; N]) -> PyResult<Self> {
        let g: G::Affine = CanonicalDeserialize::deserialize_compressed(&bytes[..])
            .map_err(serialisation_error_to_py_err)?;
        Ok(Self::Affine(g))
    }

    pub fn from_compressed_bytes_unchecked<const N: usize>(bytes: [u8; N]) -> PyResult<Self> {
        let g: G::Affine = CanonicalDeserialize::deserialize_compressed_unchecked(&bytes[..])
            .map_err(serialisation_error_to_py_err)?;
        Ok(Self::Affine(g))
    }

    pub fn msm(py: Python, points: Vec<Self>, scalars: Vec<G::ScalarField>) -> PyResult<Self> {
        py.allow_threads(|| {
            let points_affine: Vec<_> = points
                .par_iter()
                .enumerate()
                .filter_map(|(i, point)| point.as_affine().map(|p| (i, *p)))
                .collect();
            let rest: (Vec<_>, Vec<_>) = points
                .into_par_iter()
                .enumerate()
                .filter_map(|(i, point)| point.as_point().map(|p| (i, *p)))
                .unzip();
            let rest_affine = G::normalize_batch(&rest.1[..]);

            let points: BTreeMap<_, _> = points_affine
                .into_par_iter()
                .chain(rest.0.into_par_iter().zip(rest_affine))
                .collect();

            let points: Vec<_> = points.into_par_iter().map(|(_, point)| point).collect();

            let result = G::msm_unchecked(&points, &scalars);
            Ok(Self::Point(result))
        })
    }
}

#[macro_export]
macro_rules! monomorphize_point {
    ($struct: ident, $inner: ty, $scalar: ty, $COMPRESSED_SIZE: expr) => {
        #[derive(Copy, Clone)]
        #[pyclass]
        pub struct $struct($crate::point::Point<$inner>);

        #[pymethods]
        impl $struct {
            pub const COMPRESSED_SIZE: usize = $COMPRESSED_SIZE;

            /// Returns the generator of the group.
            #[new]
            fn generator() -> Self {
                Self($crate::point::Point::generator())
            }

            /// Returns the identity of the group.
            #[staticmethod]
            fn identity() -> Self {
                Self($crate::point::Point::identity())
            }

            // Overriding operators
            /// Add two elements of the group.
            fn __add__(&self, rhs: Self) -> Self {
                Self(self.0.__add__(rhs.0))
            }

            /// Subtract `rhs` from `self`.
            fn __sub__(&self, rhs: Self) -> Self {
                Self(self.0.__sub__(rhs.0))
            }

            /// Multiply `self` by a scalar from the right.
            fn __mul__(&self, rhs: $scalar) -> Self {
                Self(self.0.__mul__(rhs.0))
            }

            /// Multiply `self` by a scalar from the left.
            fn __rmul__(&self, other: $scalar) -> Self {
                Self(self.0.__mul__(other.0))
            }

            /// Negate `self`.
            fn __neg__(&self) -> Self {
                Self(self.0.__neg__())
            }

            /// Double `self`. This is usually faster than `self + self`.
            fn double(&self) -> Self {
                Self(self.0.double())
            }

            fn __str__(&self) -> PyResult<String> {
                self.0.__str__()
            }

            fn __richcmp__(&self, other: Self, op: pyclass::CompareOp) -> PyResult<bool> {
                self.0.__richcmp__(other.0, op)
            }

            /// Returns the serialized compressed bytes of `self`.
            fn to_compressed_bytes(&self) -> PyResult<[u8; $COMPRESSED_SIZE]> {
                self.0.to_compressed_bytes()
            }

            /// Deserializes a compressed point.
            #[staticmethod]
            fn from_compressed_bytes(bytes: [u8; $COMPRESSED_SIZE]) -> PyResult<Self> {
                $crate::point::Point::from_compressed_bytes(bytes).map(Self)
            }

            /// Deserializes a compressed point without checking 
            /// if it is on the curve or in the correct subgroup.
            #[staticmethod]
            fn from_compressed_bytes_unchecked(bytes: [u8; $COMPRESSED_SIZE]) -> PyResult<Self> {
                $crate::point::Point::from_compressed_bytes_unchecked(bytes).map(Self)
            }

            /// Computes the sum of `points[i] * scalars[i]`.
            #[staticmethod]
            fn msm(py: Python, points: Vec<Self>, scalars: Vec<Scalar>) -> PyResult<Self> {
                let points = points.into_iter().map(|point| point.0).collect();
                let scalars = scalars.into_iter().map(|scalar| scalar.0).collect();
                $crate::point::Point::msm(py, points, scalars).map(Self)
            }
        }
    };
}

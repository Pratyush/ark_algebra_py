#[macro_export]
macro_rules! monomorphize_poly {
    ($field: ty, $scalar: ty) => {
        use std::ops::Deref;
        use ark_poly::{
            polynomial::univariate::{DenseOrSparsePolynomial as Poly, DensePolynomial, SparsePolynomial}, DenseUVPolynomial, EvaluationDomain,
            Evaluations, Polynomial as _, Radix2EvaluationDomain,
        };

        #[derive(Clone)]
        #[pyclass]
        pub struct Domain(Radix2EvaluationDomain<$field>);

        #[pymethods]
        impl Domain {
            #[new]
            fn new(size: usize) -> Self {
                Self(Radix2EvaluationDomain::new(size).unwrap())
            }

            fn size(&self) -> usize {
                self.0.size()
            }

            fn element(&self, i: usize) -> Scalar {
                Scalar(self.0.element(i))
            }

            fn elements(&self) -> Vec<Scalar> {
                self.0.elements().map(Scalar).collect()
            }

            fn evaluate_vanishing_polynomial(&self, point: Scalar) -> Scalar {
                Scalar(self.0.evaluate_vanishing_polynomial(point.0))
            }

            fn vanishing_polynomial(&self) -> Polynomial {
                Polynomial(self.0.vanishing_polynomial().into())
            }

            fn interpolate(&self, values: Vec<Scalar>) -> Polynomial {
                let evals_on_domain =
                    Evaluations::from_vec_and_domain(values.iter().map(|v| v.0).collect(), self.0);
                Polynomial(evals_on_domain.interpolate().into())
            }
        }

        #[derive(Clone)]
        #[pyclass]
        pub struct Polynomial(Poly<'static, $field>);

        #[pymethods]
        impl Polynomial {
            /// Returns the polynomial `X`.
            #[staticmethod]
            #[allow(non_snake_case)]
            fn X() -> Self {
                Self(SparsePolynomial::from_coefficients_vec(vec![
                    (1, <$field>::one()),
                ]).into())
            }

            /// Returns the constant polynomial `c`.
            #[staticmethod]
            fn constant(c: Scalar) -> Self {
                Self(SparsePolynomial::from_coefficients_vec(vec![(0, c.0)]).into())
            }

            /// Returns the zero polynomial.
            #[staticmethod]
            fn zero() -> Self {
                Self(SparsePolynomial::zero().into())
            }

            /// Constructs a polynomial from a list of coefficients.
            #[new]
            fn from_coefficients(coeffs: Vec<Scalar>) -> Self {
                Self(DensePolynomial::from_coefficients_vec(
                    coeffs.iter().map(|c| c.0).collect(),
                ).into())
            }

            /// Returns the coefficients of the polynomial.
            fn coefficients(&self) -> Vec<Scalar> {
                DensePolynomial::from(self.0.clone()).coeffs.iter().map(|c| Scalar(*c)).collect()
            }

            /// Evaluates the polynomial at `point`.
            fn evaluate(&self, point: Scalar) -> Scalar {
                match &self.0 {
                    Poly::SPolynomial(p) => Scalar(p.evaluate(&point.0)),
                    Poly::DPolynomial(p) => Scalar(p.evaluate(&point.0)),
                }
            }

            /// Returns the degree of the polynomial.
            fn degree(&self) -> usize {
                self.0.degree()
            }

            /// Returns the quotient and remainder of self divided by the vanishing polynomial
            /// of domain.
            fn divide_by_vanishing_poly(&self, domain: Domain) -> (Self, Self) {
                self.0
                    .divide_with_q_and_r(&domain.vanishing_polynomial().0)
                    .map(|(q, r)| (Self(q.into()), Self(r.into())))
                    .unwrap()
            }

            /// Evaluates the polynomial at all elements of `domain`.
            fn evaluate_over_domain(&self, domain: Domain) -> Vec<Scalar> {
                Poly::evaluate_over_domain(self.0.clone(), domain.0).evals.into_iter().map(Scalar).collect()
            }

            // Overriding operators
            fn __add__(&self, rhs: &Self) -> Self {
                match (&self.0, &rhs.0) {
                    (Poly::SPolynomial(a), Poly::SPolynomial(b)) => Self((a.deref() + b.deref()).into()),
                    (Poly::DPolynomial(a), Poly::DPolynomial(b)) => Self((a.deref() + b.deref()).into()),
                    (Poly::SPolynomial(a), Poly::DPolynomial(b)) => Self((b.deref() + a.deref()).into()),
                    (Poly::DPolynomial(a), Poly::SPolynomial(b)) => Self((a.deref() + b.deref()).into()),
                }
            }

            fn __sub__(&self, rhs: &Self) -> Self {
                match (self.0.clone(), rhs.0.clone()) {
                    (Poly::SPolynomial(a), Poly::SPolynomial(b)) => Self((&DensePolynomial::from(a.into_owned()) - b.deref()).into()),
                    (Poly::DPolynomial(a), Poly::DPolynomial(b)) => Self((a.deref() - b.deref()).into()),
                    (Poly::SPolynomial(a), Poly::DPolynomial(b)) => Self((&DensePolynomial::from(a.into_owned()) - b.deref()).into()),
                    (Poly::DPolynomial(a), Poly::SPolynomial(b)) => Self((a.deref() - b.deref()).into()),
                }
            }

            fn __mul__(&self, rhs: Self) -> Self {
                match (&self.0, &rhs.0) {
                    (Poly::SPolynomial(a), Poly::SPolynomial(b)) => Self((a.deref().mul(b.deref()).into())),
                    (Poly::DPolynomial(a), Poly::DPolynomial(b)) => Self((a.deref() * b.deref()).into()),
                    (Poly::SPolynomial(a), Poly::DPolynomial(b)) | (Poly::DPolynomial(b), Poly::SPolynomial(a)) => Self((&DensePolynomial::from(a.clone().into_owned()) * b.deref()).into()),
                }
            }

            fn __neg__(&self) -> Self {
                match &self.0 {
                    Poly::SPolynomial(a) => Self((-a.clone().into_owned()).into()),
                    Poly::DPolynomial(a) => Self((-a.clone().into_owned()).into()),
                }
            }

            fn __truediv__(&self, rhs: Self) -> pyo3::PyResult<(Self, Self)> {
                self.0
                    .divide_with_q_and_r(&rhs.0)
                    .map(|(q, r)| (Self(q.into()), Self(r.into())))
                    .ok_or(exceptions::PyZeroDivisionError::new_err("division by zero"))
            }

            fn __repr__(&self) -> String {
                self.__str__()
            }

            fn __str__(&self) -> String {
                let mut result = String::new();
                let coeffs_iter: Box<dyn Iterator<Item = (usize, &$field)>> = match &self.0 {
                    Poly::SPolynomial(p) => Box::new(p.iter().map(|(i, c)| (*i, c))),
                    Poly::DPolynomial(p) => Box::new(p.coeffs.iter().enumerate()),
                };
                for (i, coeff) in coeffs_iter
                    .filter(|(_, c)| !c.is_zero())
                {
                    if i == 0 {
                        result += &format!("{coeff}");
                    } else if i == 1 {
                        result += &format!(" + {coeff} * x");
                    } else {
                        result += &format!(" + {coeff} * x^{i}");
                    }
                }
                result
            }

            fn __richcmp__(
                &self,
                other: Self,
                op: pyo3::pyclass::CompareOp,
            ) -> pyo3::PyResult<bool> {
                let is_eq = match (&self.0, &other.0) {
                    (Poly::SPolynomial(a), Poly::SPolynomial(b)) => a.deref() == b.deref(),
                    (Poly::DPolynomial(a), Poly::DPolynomial(b)) => a.deref() == b.deref(),
                    (Poly::SPolynomial(a), Poly::DPolynomial(b)) | (Poly::DPolynomial(b), Poly::SPolynomial(a)) => &DensePolynomial::from(a.clone().into_owned()) == b.deref(),
                };
                match op {
                    pyclass::CompareOp::Eq => Ok(is_eq),
                    pyclass::CompareOp::Ne => Ok(!is_eq),
                    _ => Err(exceptions::PyValueError::new_err(
                        "comparison operator not implemented".to_owned(),
                    )),
                }
            }
        }
    };
}

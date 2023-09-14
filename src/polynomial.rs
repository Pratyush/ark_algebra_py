#[macro_export]
macro_rules! monomorphize_poly {
    ($field: ty, $scalar: ty) => {
        use ark_poly::{
            polynomial::univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain,
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

            fn elements(&self) -> Vec<Scalar> {
                self.0.elements().map(Scalar).collect()
            }

            fn evaluate_vanishing_polynomial(&self, point: Scalar) -> Scalar {
                Scalar(self.0.evaluate_vanishing_polynomial(point.0))
            }

            fn interpolate(&self, values: Vec<Scalar>) -> Polynomial {
                let evals_on_domain =
                    Evaluations::from_vec_and_domain(values.iter().map(|v| v.0).collect(), self.0);
                Polynomial(evals_on_domain.interpolate())
            }
        }

        #[derive(Clone)]
        #[pyclass]
        pub struct Polynomial(DensePolynomial<$field>);

        #[pymethods]
        impl Polynomial {
            #[staticmethod]
            #[allow(non_snake_case)]
            fn X() -> Self {
                Self(DensePolynomial::from_coefficients_vec(vec![
                    <$field>::zero(),
                    <$field>::one(),
                ]))
            }

            #[staticmethod]
            fn constant(c: Scalar) -> Self {
                Self(DensePolynomial::from_coefficients_vec(vec![c.0]))
            }

            #[staticmethod]
            fn zero() -> Self {
                Self(DensePolynomial::zero())
            }

            #[new]
            fn from_coefficients(coeffs: Vec<Scalar>) -> Self {
                Self(DensePolynomial::from_coefficients_vec(
                    coeffs.iter().map(|c| c.0).collect(),
                ))
            }

            fn evaluate(&self, point: Scalar) -> Scalar {
                Scalar(self.0.evaluate(&point.0))
            }

            fn degree(&self) -> usize {
                self.0.degree()
            }

            /// Returns the quotient and remainder of self divided by the vanishing polynomial
            /// of domain.
            fn divide_by_vanishing_poly(&self, domain: Domain) -> (Self, Self) {
                self.0
                    .divide_by_vanishing_poly(domain.0)
                    .map(|(q, r)| (Self(q), Self(r)))
                    .unwrap()
            }

            fn evaluate_over_domain(&self, domain: Domain) -> Vec<Scalar> {
                self.0
                    .evaluate_over_domain_by_ref(domain.0)
                    .evals
                    .into_iter()
                    .map(Scalar)
                    .collect()
            }

            // Overriding operators
            fn __add__(&self, rhs: &Self) -> Self {
                Self(&self.0 + &rhs.0)
            }

            fn __sub__(&self, rhs: &Self) -> Self {
                Self(&self.0 - &rhs.0)
            }

            fn __mul__(&self, rhs: Self) -> Self {
                Self(&self.0 * &rhs.0)
            }

            fn __neg__(&self) -> Self {
                Self(-self.0.clone())
            }

            fn __str__(&self) -> pyo3::PyResult<String> {
                let mut result = String::new();
                for (i, coeff) in self
                    .0
                    .coeffs
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| !c.is_zero())
                {
                    if i == 0 {
                        result += &format!("\n{coeff}");
                    } else if i == 1 {
                        result += &format!(" + \n{coeff} * x");
                    } else {
                        result += &format!(" + \n{coeff} * x^{i}");
                    }
                }
                Ok(result)
            }

            fn __richcmp__(
                &self,
                other: Self,
                op: pyo3::pyclass::CompareOp,
            ) -> pyo3::PyResult<bool> {
                match op {
                    pyclass::CompareOp::Eq => Ok(self.0 == other.0),
                    pyclass::CompareOp::Ne => Ok(self.0 != other.0),
                    _ => Err(exceptions::PyValueError::new_err(
                        "comparison operator not implemented".to_owned(),
                    )),
                }
            }
        }
    };
}

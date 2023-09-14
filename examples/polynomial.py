from ark_algebra_py.ark_algebra_py import Polynomial, Scalar, Domain


# Initilisation -- This is the generator point
a = Polynomial([Scalar(100), Scalar(10), Scalar(1)])
b = Polynomial([Scalar(10), Scalar(5), Scalar(1)])

print(a + b)
print(a - b)
print(a * b)
print(a * b)

assert a.degree() == 2
assert b.degree() == 2
assert (a + b).degree() == 2

a_at_2 = a.evaluate(Scalar(2))
b_at_2 = b.evaluate(Scalar(2))

assert a_at_2 == Scalar(124)
assert b_at_2 == Scalar(24)
assert a_at_2 + b_at_2 == (a + b).evaluate(Scalar(2))
assert a_at_2 * b_at_2 == (a * b).evaluate(Scalar(2))

domain = Domain(8)
a = domain.interpolate([Scalar(1), Scalar(2), Scalar(3)])

assert(a.evaluate(domain.element(0)) == Scalar(1))
assert(a.evaluate(domain.element(1)) == Scalar(2))
assert(a.evaluate(domain.element(2)) == Scalar(3))

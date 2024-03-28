# `arkworks` BLS12-381 bindings

The main usage of this library at this moment is to generate test vectors for EIP4844 in the [consensus-specs](https://github.com/ethereum/consensus-specs/tree/master). The library itself is generic, so feel free to use it for other purposes.

## Usage examples

### Scalar field arithmetic

```python
from ark_algebra_py.ark_algebra_py import Scalar

# Initialisation - The default initialiser for a scalar is an u128 integer
scalar = Scalar(12345)

# Equality -- We override eq and neq operators
assert(scalar == scalar)
assert(Scalar(1234) != Scalar(4567))

# Scalar Addition/subtraction/Negation -- We override the add/sub/neg operators
a = Scalar(3)
b = Scalar(4)
c = Scalar(5)
assert(a.square() + b.square() == c.square())
assert(a * a + b * b == c * c)

neg_a = -a
assert(a + neg_a == Scalar(0))
assert(a + neg_a).is_zero()

# Serialisation
compressed_bytes = scalar.to_le_bytes()
deserialised_scalar = Scalar.from_le_bytes(compressed_bytes)
assert(scalar == deserialised_scalar)
```

### Group arithmetic

```python
from ark_algebra_py.ark_algebra_py import G1, G2, Scalar

# G1 and G2 have the same methods implemented on them
# For brevity, we will only see one method using G1 and G2;
# the rest of the code will just use G1

# Point initialization -- This will be initialized to the generator of G1.
g1_generator = G1()
g2_generator = G2()

# Identity element 
identity = G1.identity()

# Equality -- We override eq and neq operators
assert(g1_generator == g1_generator)
assert(g1_generator != identity)


# Printing an element -- We override __str__ so when we print
# an element it prints in hex
print("identity: ",identity)
print("g1 generator: ", g1_generator)
print("g2 generator: ", g2_generator)

# Point Addition/subtraction/Negation -- We override the add/sub/neg operators
gen = G1()
double_gen = gen + gen
double_gen2 = gen.double()
assert(double_gen == double_gen2)
assert((double_gen2 - gen) == gen)
neg_gen = -gen
assert(neg_gen + gen == identity)

# Scalar multiplication
scalar = Scalar(4)
four_gen = gen * scalar
four_gen_2 = scalar * gen 
assert(four_gen == gen + gen + gen + gen)
assert(four_gen == four_gen_2)

# Serialisation
# 
# serialising to/from a g1 point
# We don't expose the uncompressed form 
# because it seems like its not needed
compressed_bytes = gen.to_compressed_bytes()
deserialised_point = G1.from_compressed_bytes(compressed_bytes)
# If the bytes being received are trusted, we can avoid
# doing subgroup checks
deserialised_point_unchecked = G1.from_compressed_bytes_unchecked(compressed_bytes)
assert(deserialised_point == deserialised_point_unchecked)
assert(deserialised_point == gen)

```

### Pairings

```python
from ark_algebra_py.ark_algebra_py import G1, G2, PairingOutput, Scalar, Pairing


# Initilisation -- This is the generator point
gt_gen = PairingOutput()

# Zero/One
zero = PairingOutput.one()

# Computing a pairing using pairing and multi_pairing
# multi_pairing does multiple pairings and adds them together with only one final_exp
assert gt_gen == Pairing.pairing(G1(), G2()) 
g1s = [G1()]
g2s = [G2()]
assert gt_gen == Pairing.multi_pairing(g1s, g2s)

# Bilinearity
a = Scalar(1234)
b = Scalar(4566)
c = a * b


g = G1() * a
h = G2() * b

p = Pairing.pairing(g, h)

c_g1 = G1() *c
c_g2 = G2() *c

assert p == Pairing.pairing(c_g1, G2())
assert p == Pairing.pairing(G1(), c_g2)
```

## Development

We use `maturin` to build the python bindings. To build the bindings, run the following command:

```bash
source .env/bin/activate
```

If you are using the `fish` shell, you can run the following command:

```bash
source .env/bin/activate.fish
```

After activating the virtual environment, you can run the following command to build the bindings:

```bash
maturin develop
```

This will build the bindings and install the package in the current virtual environment.

Once you are done making changes, commit and push the changes to the repository. The CI will run the tests and build the bindings for all platforms.

If you additionally want to trigger a PyPi release, you can create a new tag and push it to the repository with `git push --tags`. The CI will automatically build the bindings and publish the package to PyPi.

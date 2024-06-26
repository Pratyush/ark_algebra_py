from ark_algebra_py.ark_algebra_py import G1, G2, Scalar

# G1 and G2 have the same methods implemented on them
# For brevity, I will only show one method using G1 and G2 
# The rest of the code will just use G1

# Point initialization -- This will be initialized to the g1 generator 
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

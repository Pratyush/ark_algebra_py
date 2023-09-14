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

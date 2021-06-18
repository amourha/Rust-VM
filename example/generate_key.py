from z3 import *

k = [
  BitVec("k[0]", 32),
  BitVec("k[1]", 32),
  BitVec("k[2]", 32),
  BitVec("k[3]", 32)
]

s = Solver()
s.add(k[0] ^ k[3] == 0x3713)
s.add((k[0] & 0x1414) == 0)
s.add(k[1] | k[2] == 0x7f7f)

print(s.check())

m = s.model()


print ("%.8x "%(m[k[0]].as_long()) +
 "%.8x "% (m[k[1]].as_long()) +
 "%.8x "% (m[k[2]].as_long()) +
 "%.8x "% (m[k[3]].as_long()))

